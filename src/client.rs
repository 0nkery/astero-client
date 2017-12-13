use std::io;
use std::io::{Cursor, Write, Read};
use std::iter::repeat;
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, IpAddr};
use std::thread;
use std::time::Duration;
use std::sync::mpsc as std_mpsc;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use futures::{Stream, Sink, Future, stream};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::{Core, Timeout};


#[derive(Debug)]
pub enum Msg {
    // helper messages (for internal game client usage)
    Unknown,
    ServerNotResponding,

    // client messages
    Join(String),
    Leave,
    ClientHeartbeat,

    // server messages
    JoinAck(u16, f32, f32),
    OtherJoined(u16, String, f32, f32),
    OtherLeft(u16),
    ServerHeartbeat,
}

impl Msg {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        let mut rdr = Cursor::new(buf);

        let msg = match rdr.read_i16::<BigEndian>()? {
            0 => {
                let id = rdr.read_u16::<BigEndian>()?;
                let x = rdr.read_i16::<BigEndian>()? as f32;
                let y = rdr.read_i16::<BigEndian>()? as f32;

                Msg::JoinAck(id, x, y)
            },

            1 => {
                let conn_id = rdr.read_u16::<BigEndian>()?;
                let nickname_length = rdr.read_u8()? as usize;

                let mut nickname = Vec::with_capacity(nickname_length);
                let nickname = unsafe {
                    nickname.set_len(nickname_length);
                    rdr.read_exact(nickname.as_mut_slice())?;
                    String::from_utf8_unchecked(nickname)
                };

                let x = rdr.read_i16::<BigEndian>()? as f32;
                let y = rdr.read_i16::<BigEndian>()? as f32;

                Msg::OtherJoined(conn_id, nickname, x, y)
            }

            2 => {
                let conn_id = rdr.read_u16::<BigEndian>()?;

                Msg::OtherLeft(conn_id)
            }

            3 => Msg::ServerHeartbeat,

            _ => Msg::Unknown
        };

        Ok(msg)
    }

    pub fn into_bytes(self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self {
            Msg::Join(nickname) => {
                buf.write_u16::<BigEndian>(0)?;
                buf.write_u8(nickname.len() as u8)?;
                buf.write_all(nickname.as_bytes())?;
            }

            Msg::Leave => {
                buf.write_u16::<BigEndian>(1)?;
            }

            Msg::ClientHeartbeat => {
                buf.write_u16::<BigEndian>(2)?;
            }

            _ => {}
        }

        Ok(())
    }
}


struct ClientCodec {
    server: SocketAddr
}

impl ClientCodec {
    pub fn new() -> Self {
        let server = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::localhost(), 11111, 0, 0));

        ClientCodec {
            server
        }
    }
}

impl UdpCodec for ClientCodec {
    type In = Msg;
    type Out = Msg;

    fn decode(&mut self, src: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        if *src != self.server {
            return Ok(Msg::Unknown);
        }

        Msg::from_bytes(buf)
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> SocketAddr {
        msg.into_bytes(buf).expect("Failed to serialize message");

        self.server
    }
}


pub struct Client {
    thread_handle: Option<thread::JoinHandle<()>>,
    to: Option<futures_mpsc::UnboundedSender<Msg>>,
    from: std_mpsc::Receiver<Msg>,
    timeouts: u32,
}

impl Client {
    pub fn start() -> Self {
        let (to_main_thread, from_client) = std_mpsc::channel();
        let (to_client, from_main_thread) = futures_mpsc::unbounded();

        let thread_handle = thread::spawn(move || {
            let mut reactor = Core::new().expect("Failed to init reactor");
            let handle = reactor.handle();

            let client_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::localhost()), 0);
            let socket =
                UdpSocket::bind(&client_address, &handle)
                    .expect("Failed to create socket");

            let (outgoing, ingoing) =
                socket.framed(ClientCodec::new()).split();

            // Stream of timeouts. Selected with network messages.
            // If timeout comes first it means that server is not sending any data.
            let timeout_handle = handle.clone();
            let timeouts =
                stream::iter_result::<_, _, ()>(repeat(Ok(())))
                    .and_then(move |()|
                        Timeout::new(Duration::new(6, 0), &timeout_handle)
                            .expect("Failed to setup timeout")
                            .map_err(|err| panic!("{}", err))
                    )
                    .map(|_| Msg::ServerNotResponding)
                    .map_err(|_| io::ErrorKind::Other.into());

            let ingoing = ingoing.select(timeouts);

            let receiver = ingoing.for_each(move |msg| {
                to_main_thread.send(msg).expect("Failed to drop message to the main thread");
                Ok(())
            }).map_err(|err| panic!("{}", err));

            // Receiver is spawned separately. We don't care much about received packets
            // when we're going to exit this thread.
            handle.spawn(receiver);

            let from_main_thread = from_main_thread
                .map_err(|_err| -> io::Error {
                    io::ErrorKind::Other.into()
                });

            // But `from_main_thread` stream should terminate in order to shutdown this thread.
            let sender = outgoing.send_all(from_main_thread);

            reactor.run(sender).expect("Client thread failure");
        });

        Client {
            thread_handle: Some(thread_handle),
            to: Some(to_client),
            from: from_client,
            timeouts: 0
        }
    }

    pub fn stop(&mut self) {
        self.send(Msg::Leave);

        // Dropping `to` causes Sink-part of UdpFramed to flush all pending packets and exit.
        let sender = self.to.take().unwrap();
        drop(sender);

        self.thread_handle
            .take()
            .unwrap()
            .join()
            .expect("Failed to stop client thread");
    }

    pub fn send(&self, msg: Msg) {
        self.to.as_ref().unwrap().unbounded_send(msg)
            .expect("Failed to drop message to client thread");
    }

    pub fn try_recv(&mut self) -> Result<Msg, std_mpsc::TryRecvError> {
        match self.from.try_recv() {
            Ok(Msg::Unknown) => self.try_recv(),

            Ok(Msg::ServerNotResponding) => {
                self.timeouts += 1;
                println!("Got ServerNotResponding. Timeouts - {}", self.timeouts);
                if self.timeouts >= 3 {
                    return Ok(Msg::ServerNotResponding);
                }

                self.try_recv()
            }

            Ok(msg) => {
                self.timeouts = 0;
                match msg {
                    Msg::ServerHeartbeat => {
                        self.send(Msg::ClientHeartbeat);
                        self.try_recv()
                    }

                    _ => Ok(msg)
                }
            }

            other => other
        }
    }
}
