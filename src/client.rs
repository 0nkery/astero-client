use std;
use std::io;
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, IpAddr};
use std::time::Duration;
use std::thread;

use futures;
use tokio_core::{
    net::UdpSocket,
    reactor::Core,
    reactor::Interval,
};

use proto::{
    astero,
    mmob,
};
use util::cur_time_in_millis;


use futures::{
    Stream,
    Sink,
    Future,
};
use prost::Message;
use tokio_core::net::UdpCodec;


#[derive(Debug)]
pub enum Msg {
    // helper messages (for internal game client usage)
    Unknown,
    ServerNotResponding,

    JoinGame(String),
    JoinAck(astero::Player),
    LeaveGame,
    Heartbeat,
    Latency(mmob::LatencyMeasure),

    ToServer(astero::client::Msg),
    FromServer(astero::server::Msg),
}

impl Msg {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        let msg = mmob::Server::decode(buf);

        let msg = match msg {
            Err(..) => Msg::Unknown,
            Ok(msg) => {
                let msg = msg.msg.unwrap();
                match msg {
                    mmob::server::Msg::Heartbeat(..) => Msg::Heartbeat,
                    mmob::server::Msg::LatencyMeasure(measure) => Msg::Latency(measure),
                    mmob::server::Msg::JoinAck(ack) => {
                        if let Some(payload) = ack.payload {
                            let player = astero::Player::decode(payload)
                                .expect("Failed to decode player data");

                            Msg::JoinAck(player)
                        } else {
                            Msg::Unknown
                        }
                    }
                    mmob::server::Msg::Proxied(msg) => {
                        let msg = astero::Server::decode(msg.msg)
                            .expect("Failed to decode proxied message");

                        if let Some(msg) = msg.msg {
                            Msg::FromServer(msg)
                        } else {
                            Msg::Unknown
                        }
                    }
                }
            },
        };

        Ok(msg)
    }
}


struct ClientCodec {
    server: SocketAddr,
    buf: Vec<u8>,
}

impl ClientCodec {
    pub fn new() -> Self {
        let server = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::localhost(), 11_111, 0, 0));

        ClientCodec {
            server,
            buf: Vec::new(),
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
        let msg = match msg {
            Msg::JoinGame(nickname) => {
                let payload = astero::JoinPayload { nickname };
                payload.encode(&mut self.buf)
                    .expect("Failed to write JoinPayload");

                mmob::client::Msg::Join(mmob::JoinGame {
                    payload: Some(self.buf.clone()),
                })
            }
            Msg::LeaveGame => mmob::client::Msg::Leave(mmob::LeaveGame {}),
            Msg::Heartbeat => mmob::client::Msg::Heartbeat(mmob::Heartbeat {}),
            Msg::Latency(measure) => mmob::client::Msg::LatencyMeasure(measure),
            Msg::ToServer(msg) => {
                msg.encode(&mut self.buf);

                mmob::client::Msg::Proxied(mmob::Proxied {
                    msg: self.buf.clone()
                })
            }

            Msg::Unknown |
            Msg::ServerNotResponding |
            Msg::JoinAck(..) |
            Msg::FromServer(..) => unreachable!()
        };

        let msg = mmob::Client { msg: Some(msg) };
        msg.encode(buf)
            .expect("Failed to encode message");

        self.buf.clear();

        self.server
    }
}


pub struct ClientHandle {
    thread_handle: Option<thread::JoinHandle<()>>,
    to: Option<futures::sync::mpsc::UnboundedSender<Msg>>,
    from: std::sync::mpsc::Receiver<Msg>,
    stop: Option<futures::sync::oneshot::Sender<()>>,
    timeouts: u32,
}

impl ClientHandle {
    pub fn start() -> Self {
        let (to_main_thread, from_client) = std::sync::mpsc::channel();
        let (to_client, from_main_thread) = futures::sync::mpsc::unbounded();
        let (stop_sender, stop_receiver) = futures::sync::oneshot::channel();

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
            let timeouts =
                Interval::new(Duration::new(6, 0), &handle)
                .expect("Failed to setup timeouts")
                .map(|_| Msg::ServerNotResponding);

            let ingoing = ingoing.select(timeouts);

            let receiver = ingoing.for_each(move |msg| {
                to_main_thread.send(msg).expect("Failed to drop message to the main thread");
                Ok(())
            }).map_err(|err| panic!("{}", err));

            let from_main_thread = from_main_thread
                .map_err(|_err| -> io::Error {
                    io::ErrorKind::Other.into()
                });

            let latency_measures =
                Interval::new(Duration::new(1, 0), &handle)
                    .expect("Failed to setup latency measures")
                    .map(|_| {
                        Msg::Latency(mmob::LatencyMeasure {
                            timestamp: cur_time_in_millis()
                        })
                    });

            let sender = outgoing.send_all(from_main_thread.select(latency_measures));

            let client = sender.join(receiver).select2(stop_receiver);
            reactor.run(client).ok().expect("Client thread failure");
        });

        ClientHandle {
            thread_handle: Some(thread_handle),
            to: Some(to_client),
            from: from_client,
            stop: Some(stop_sender),
            timeouts: 0
        }
    }

    pub fn stop(&mut self) {
        self.send(Msg::LeaveGame);

        self.stop
            .take()
            .expect("Absent stop sender?! What?!")
            .send(())
            .expect("Failed to stop client reactor");

        self.thread_handle
            .take()
            .expect("Absent thread handle?! What?")
            .join()
            .expect("Failed to join client thread");
    }

    pub fn send(&self, msg: Msg) {
        self.to
            .as_ref()
            .and_then(|s| {
                s.unbounded_send(msg).expect("Failed to send message to client thread");
                Some(())
            });
    }

    pub fn try_recv(&mut self) -> Result<Msg, std::sync::mpsc::TryRecvError> {
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

                    _ => Ok(msg)
                }
            }

            other => other
        }
    }
}
