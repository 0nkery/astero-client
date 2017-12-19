use std::borrow::Cow;

use util::Point2;

pub use super::proto_defs::astero::{
    Client,
    mod_Client,

    Join,
    Leave,
    Heartbeat,

    Coord,

    mod_Server,
    Server,

    JoinAck,
};

use super::proto_defs::astero::{
    OtherJoined,
};


impl<'a> Join<'a> {
    pub fn new(nickname: String) -> Client<'a> {
        let join = Join {
            nickname: Cow::from(nickname),
        };

        Client {
            msg: mod_Client::OneOfmsg::join(join)
        }
    }
}


impl Leave {
    pub fn new<'a>() -> Client<'a> {
        let leave = Leave { };

        Client {
            msg: mod_Client::OneOfmsg::leave(leave)
        }
    }
}


impl Heartbeat {
    pub fn new<'a>() -> Client<'a> {
        let heartbeat = Heartbeat { };

        Client {
            msg: mod_Client::OneOfmsg::heartbeat(heartbeat)
        }
    }
}


impl Into<Point2> for Coord {
    fn into(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}


#[derive(Debug)]
pub struct OtherData {
    pub id: u32,
    pub nickname: String,
    pub pos: Point2,
}

impl<'a> Into<OtherData> for OtherJoined<'a> {
    fn into(self) -> OtherData {
        OtherData {
            id: self.id,
            nickname: self.nickname.to_string(),
            pos: self.pos.into()
        }
    }
}
