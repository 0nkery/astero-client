mod proto_defs;
pub mod proto;
mod body;

use std::io;
use std::iter::repeat;
use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, IpAddr};
use std::thread;
use std::time::Duration;
use std::sync::mpsc as std_mpsc;

use futures::{Stream, Sink, Future, stream};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::{Core, Timeout};
use quick_protobuf::{Writer, BytesReader, MessageWrite, MessageRead};

use client::proto::{
    Join,
    Leave,
    Heartbeat,
    JoinAck,
    mod_Server,
    Server,
    OtherData,
    OtherLeft,
    Spawn,
    SimUpdate,
    Input,
    OtherInput,
    LatencyMeasure,
};


#[derive(Debug)]
pub enum Msg {
    // helper messages (for internal game client usage)
    Unknown,
    ServerNotResponding,

    // both client & server
    Heartbeat,

    // client messages
    Join(String),
    Leave,
    Input(Input),
    Latency(LatencyMeasure),

    // server messages
    JoinAck(JoinAck),
    OtherJoined(OtherData),
    OtherLeft(OtherLeft),
    Spawn(Spawn),
    SimUpdates(Vec<SimUpdate>),
    OtherInput(OtherInput),
}

impl Msg {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        let mut rdr = BytesReader::from_bytes(buf);
        let msg = Server::from_reader(&mut rdr, &buf);

        let msg = match msg {
            Err(..) => Msg::Unknown,
            Ok(msg) => msg.into(),
        };

        Ok(msg)
    }

    pub fn into_bytes(self, buf: &mut Vec<u8>) -> io::Result<()> {
        let msg = match self {
            Msg::Join(nickname) => Some(Join::new(nickname)),
            Msg::Leave => Some(Leave::new()),
            Msg::Heartbeat => Some(Heartbeat::new()),
            Msg::Input(input) => Some(Input::new(input)),
            Msg::Latency(latency) => Some(LatencyMeasure::new(latency)),

            _ => None
        };

        if let Some(msg) = msg {
            let mut writer = Writer::new(buf);
            msg.write_message(&mut writer).map_err(|e| -> io::Error { e.into() })?;
        }

        Ok(())
    }
}


impl<'a> From<Server<'a>> for Msg {
    fn from(msg_from_server: Server<'a>) -> Self {
        let msg = msg_from_server.msg;

        match msg {
            mod_Server::OneOfmsg::join_ack(ack) => Msg::JoinAck(ack),
            mod_Server::OneOfmsg::other_joined(other) => Msg::OtherJoined(other.into()),
            mod_Server::OneOfmsg::other_left(other) => Msg::OtherLeft(other),
            mod_Server::OneOfmsg::heartbeat(..) => Msg::Heartbeat,
            mod_Server::OneOfmsg::spawn(spawn) => Msg::Spawn(spawn),
            mod_Server::OneOfmsg::sim_updates(list_of) => Msg::SimUpdates(list_of.updates),
            mod_Server::OneOfmsg::other_input(input) => Msg::OtherInput(input),

            mod_Server::OneOfmsg::None => Msg::Unknown,
        }
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


pub struct ClientHandle {
    thread_handle: Option<thread::JoinHandle<()>>,
    to: Option<futures_mpsc::UnboundedSender<Msg>>,
    from: std_mpsc::Receiver<Msg>,
    timeouts: u32,
}

impl ClientHandle {
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
        self.send(Msg::Leave);

        // Dropping `to` causes Sink-part of UdpFramed to flush all pending packets and exit.
        let sender = self.to.take().expect("Failed to flush all pending packets");
        drop(sender);

        self.thread_handle
            .take()
            .expect("Absent thread handle?! What?")
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
