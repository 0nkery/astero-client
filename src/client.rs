use std::io;
use std::io::Write;
use std::net::{SocketAddr, Ipv6Addr, IpAddr};
use std::thread;
use std::sync::mpsc as std_mpsc;

use byteorder::{BigEndian, WriteBytesExt};
use futures::{Stream, Sink, Future};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::Core;


pub enum Msg {
    Join(String)
}

impl Msg {
    pub fn from_bytes(buf: &[u8]) -> io::Result<Self> {
        unimplemented!()
    }

    pub fn into_bytes(self, buf: &mut Vec<u8>) -> io::Result<()> {
        match self {
            Msg::Join(username) => {
                buf.write_u16::<BigEndian>(0)?;
                buf.write_u8(username.len() as u8)?;
                buf.write_all(username.as_bytes())?;
            }
        }

        Ok(())
    }
}


struct ClientCodec {
    server: SocketAddr
}

impl ClientCodec {
    pub fn new(server: SocketAddr) -> Self {
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
            return Err(io::ErrorKind::InvalidInput.into());
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
        let (from_client_tx, from_client_rx) = std_mpsc::channel();
        let (to_client_tx, to_client_rx) = futures_mpsc::unbounded();

        let thread_handle = thread::spawn(move || {
            let mut reactor = Core::new().expect("Failed to init reactor");
            let handle = reactor.handle();

            let client_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::unspecified()), 0);
            let socket =
                UdpSocket::bind(&client_address, &handle)
                    .expect("Failed to create socket");

            let server_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(
                0xfe80, 0, 0, 0, 0x0224, 0x1dff, 0xfe7f, 0x5b83
            )), 11111);

            let (mut outgoing, ingoing) =
                socket.framed(ClientCodec::new(server_address)).split();
            let outgoing_ref = &mut outgoing;

            let receiver = ingoing.for_each(|msg| {
                from_client_tx.send(msg).expect("Failed to drop message to the main thread");
                Ok(())
            });

            let sender =
                to_client_rx.for_each(|msg| {
                    outgoing_ref.send(msg).wait().expect("Failed to send UDP packet");
                    Ok(())
                }).map_err(|_err| io::ErrorKind::Other.into());

            let client = sender.join(receiver);
            reactor.run(client).expect("Failed to start client");
        });

        Client {
            thread_handle,
            to: to_client_tx,
            from: from_client_rx
        }
    }

    pub fn stop(&mut self) {

    }
}
