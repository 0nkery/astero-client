use std::io;
use std::net::{SocketAddr, Ipv6Addr, IpAddr};
use std::thread;
use std::sync::mpsc as std_mpsc;

use futures::{Stream, Sink, Future};
use futures::sync::mpsc as futures_mpsc;
use tokio_core::net::{UdpCodec, UdpSocket};
use tokio_core::reactor::Core;


pub enum Msg {

}


struct ClientCodec;

impl ClientCodec {
    pub fn new() -> Self {
        ClientCodec {}
    }
}

impl UdpCodec for ClientCodec {
    type In = Msg;
    type Out = Msg;

    fn decode(&mut self, src: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        unimplemented!()
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> SocketAddr {
        unimplemented!()
    }
}


pub fn start_client() -> (thread::JoinHandle<()>, futures_mpsc::UnboundedSender<Msg>, std_mpsc::Receiver<Msg>) {
    let (from_client_tx, from_client_rx) = std_mpsc::channel();
    let (to_client_tx, to_client_rx) = futures_mpsc::unbounded();

    let thread_handle = thread::spawn(move || {
        let server_address =
            "fe80::224:1dff:fe7f:5b83"
                .parse::<SocketAddr>()
                .expect("Failed to parse server address");

        let mut reactor = Core::new().expect("Failed to init reactor");
        let handle = reactor.handle();

        let client_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::unspecified()), 0);
        let socket =
            UdpSocket::bind(&client_address, &handle)
                .expect("Failed to create socket");
        let (mut outgoing, ingoing) =
            socket.framed(ClientCodec::new()).split();
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

    (thread_handle, to_client_tx, from_client_rx)
}