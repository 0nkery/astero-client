use std::borrow::Cow;

use super::proto_defs::astero::{
    Client,
    mod_Client,

    Join,
    Leave,
    Heartbeat,
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