use std::borrow::Cow;
use std::io;
use std::iter::repeat;
use std::marker::PhantomData;
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, IpAddr};
use std::sync::mpsc as std_mpsc;
use std::time::Duration;
use std::thread;

use futures::{Stream, Sink, Future, stream};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::{Core, Timeout};
use quick_protobuf::{Writer, BytesReader, MessageWrite, MessageRead};

use proto::astero;
use proto::mmob;
use proto::AsteroServerMsg;
use proto::MmobClientMsg;


#[derive(Debug)]
pub enum Msg<'a> {
    // helper messages (for internal game client usage)
    Unknown,
    ServerNotResponding,

    JoinGame(String),
    JoinAck(astero::Player<'a>),
    LeaveGame,
    Heartbeat,
    Latency(mmob::LatencyMeasure),

    ToServer(astero::Client),
    FromServer(AsteroServerMsg<'a>),
}

impl<'a> Msg<'a> {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        let mut rdr = BytesReader::from_bytes(buf);
        let msg = mmob::Server::from_reader(&mut rdr, &buf);

        let msg = match msg {
            Err(..) => Msg::Unknown,
            Ok(msg) => {
                match msg {

                }
            },
        };

        Ok(msg)
    }
}


struct ClientCodec<'a> {
    server: SocketAddr,
    buf: Vec<u8>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ClientCodec<'a> {
    pub fn new() -> Self {
        let server = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::localhost(), 11111, 0, 0));

        ClientCodec {
            server,
            buf: Vec::new(),
            phantom: PhantomData {}
        }
    }
}

impl<'a> UdpCodec for ClientCodec<'a> {
    type In = Msg<'a>;
    type Out = Msg<'a>;

    fn decode(&mut self, src: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        if *src != self.server {
            return Ok(Msg::Unknown);
        }

        Msg::from_bytes(buf)
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> SocketAddr {
        let msg = match msg {
            Msg::JoinGame(nickname) => {
                let payload = astero::JoinPayload {
                    nickname: Cow::from(nickname)
                };
                let mut writer = Writer::new(self.buf);
                payload.write_message(&mut writer)
                    .expect("Failed to write JoinPayload");

                MmobClientMsg::join(mmob::JoinGame {
                    payload: Some(Cow::from(self.buf)),
                })
            }
            Msg::LeaveGame => MmobClientMsg::leave(mmob::LeaveGame {}),
            Msg::Heartbeat => MmobClientMsg::heartbeat(mmob::Heartbeat {}),
            Msg::Latency(measure) => MmobClientMsg::latency_measure(measure),
        };

        let msg = mmob::Client { Msg: msg };
        let mut writer = Writer::new(buf);
        msg.write_message(&mut writer)
            .expect("Failed to encode message");

        self.buf.clear();

        self.server
    }
}


pub struct ClientHandle<'a> {
    thread_handle: Option<thread::JoinHandle<()>>,
    to: Option<futures_mpsc::UnboundedSender<Msg<'a>>>,
    from: std_mpsc::Receiver<Msg<'a>>,
    timeouts: u32,
}

impl<'a> ClientHandle<'a> {
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

        ClientHandle {
            thread_handle: Some(thread_handle),
            to: Some(to_client),
            from: from_client,
            timeouts: 0
        }
    }

    pub fn stop(&mut self) {
        self.send(Msg::LeaveGame);

        // Dropping `to` causes Sink-part of UdpFramed to flush all pending packets and exit.
        let sender = self.to.take().expect("Failed to flush all pending packets");
        drop(sender);

        self.thread_handle
            .take()
            .expect("Absent thread handle?! What?")
            .join()
            .expect("Failed to stop client thread");
    }

    pub fn send(&self, msg: Msg<'a>) {
        self.to
            .as_ref()
            .and_then(|s| {
                s.unbounded_send(msg).expect("Failed to send message to client thread");
                Some(())
            });
    }

    pub fn try_recv(&mut self) -> Result<Msg, std_mpsc::TryRecvError> {
        match self.from.try_recv() {
            Ok(Msg::Unknown) => self.try_recv(),

            Ok(Msg::ServerNotResponding) => {
                self.timeouts += 1;
                if self.timeouts >= 3 {
                    return Ok(Msg::ServerNotResponding);
                }

                self.try_recv()
            }

            Ok(msg) => {
                self.timeouts = 0;
                match msg {
                    Msg::Heartbeat => {
                        self.send(Msg::Heartbeat);
                        self.try_recv()
                    }
                    Msg::Latency(latency) => {
                        self.send(Msg::Latency(latency));
                        self.try_recv()
                    }

                    _ => Ok(msg)
                }
            }

            other => other
        }
    }
}
