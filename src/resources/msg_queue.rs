use std::collections::VecDeque;

use msg::Msg;


pub struct MsgQueue(pub VecDeque<Msg>);
