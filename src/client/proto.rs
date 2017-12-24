use std::borrow::Cow;

use ggez::graphics::{Point2, Vector2};

pub use super::body::Body;

pub use super::proto_defs::astero::{
    Client,
    mod_Client,

    Join,
    Leave,
    Heartbeat,

    Coord,
    Asteroid,
    Body as ProtoBody,
    Entity,
    SimUpdate,

    mod_Server,
    Server,

    JoinAck,
    OtherLeft,
    Spawn,
    mod_Spawn::OneOfentity as SpawnEntity,
    SimUpdates,
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


impl Copy for Coord {}

impl Into<Point2> for Coord {
    fn into(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}

impl From<Point2> for Coord {
    fn from(point: Point2) -> Self {
        Coord { x: point.x, y: point.y }
    }
}

impl Into<Vector2> for Coord {
    fn into(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl From<Vector2> for Coord {
    fn from(vec: Vector2) -> Self {
        Coord { x: vec.x, y: vec.y }
    }
}


#[derive(Debug)]
pub struct OtherData {
    pub id: u32,
    pub nickname: String,
    pub body: ProtoBody,
}

impl<'a> Into<OtherData> for OtherJoined<'a> {
    fn into(self) -> OtherData {
        OtherData {
            id: self.id,
            nickname: self.nickname.to_string(),
            body: self.body,
        }
    }
}
