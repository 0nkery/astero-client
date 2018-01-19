use std::io;

use proto::{
    astero,
    mmob,
};


use prost::Message;


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
                let msg = msg.msg.expect("Got empty mmob message from server");
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