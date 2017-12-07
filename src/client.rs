use std::io;
use std::io::{Cursor, Write, Read};
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, IpAddr};
use std::thread;
use std::sync::mpsc as std_mpsc;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use futures::{Stream, Sink, Future};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::Core;


#[derive(Debug)]
pub enum Msg {
    // helper messages
    Unknown,
    // client messages
    Join(String),

    // server messages
    JoinAck(u16),
    OtherJoined(u16, String),
}

impl Msg {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        let mut rdr = Cursor::new(buf);

        match rdr.read_i16::<BigEndian>()? {
            0 => Ok(Msg::JoinAck(rdr.read_u16::<BigEndian>()?)),

            1 => {
                let conn_id = rdr.read_u16::<BigEndian>()?;
                let nickname_length = rdr.read_u8()?;
                let mut nickname = String::with_capacity(nickname_length as usize);
                unsafe {
                    rdr.read_exact(nickname.as_bytes_mut())?;
                }

                Ok(Msg::OtherJoined(conn_id, nickname))
            }

            _ => Ok(Msg::Unknown),
        }
    }

    pub fn into_bytes(self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self {
            Msg::Join(nickname) => {
                buf.write_u16::<BigEndian>(0)?;
                buf.write_u8(nickname.len() as u8)?;
                buf.write_all(nickname.as_bytes())?;
            },

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
        let addr = Ipv6Addr::new(0xfe80, 0, 0, 0, 0x0224, 0x1dff, 0xfe7f, 0x5b83);
        let server = SocketAddr::V6(SocketAddrV6::new(addr, 11111, 0, 2));

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
    thread_handle: thread::JoinHandle<()>,
    pub to: futures_mpsc::UnboundedSender<Msg>,
    pub from: std_mpsc::Receiver<Msg>
}

impl Client {
    pub fn start() -> Self {
        let (to_main_thread, from_client) = std_mpsc::channel();
        let (to_client, from_main_thread) = futures_mpsc::unbounded();

        let thread_handle = thread::spawn(move || {
            let mut reactor = Core::new().expect("Failed to init reactor");
            let handle = reactor.handle();

            let client_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::unspecified()), 0);
            let socket =
                UdpSocket::bind(&client_address, &handle)
                    .expect("Failed to create socket");

            let (outgoing, ingoing) =
                socket.framed(ClientCodec::new()).split();

            let receiver = ingoing.for_each(|msg| {
                to_main_thread.send(msg).expect("Failed to drop message to the main thread");
                Ok(())
            });

            let from_main_thread = from_main_thread
                .map_err(|_| -> io::Error { io::ErrorKind::Other.into() });

            let sender = outgoing.send_all(from_main_thread);

            let client = sender.join(receiver);
            reactor.run(client).expect("Failed to start client");
        });

        Client {
            thread_handle,
            to: to_client,
            from: from_client
        }
    }

    pub fn stop(&mut self) {

    }
}
