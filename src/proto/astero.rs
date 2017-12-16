//! Automatically generated rust module for 'astero.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Coord {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl<'a> MessageRead<'a> for Coord {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = Some(r.read_float(bytes)?),
                Ok(21) => msg.y = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Coord {
    fn get_size(&self) -> usize {
        0
        + self.x.as_ref().map_or(0, |_| 1 + 4)
        + self.y.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.x { w.write_with_tag(13, |w| w.write_float(*s))?; }
        if let Some(ref s) =self.y { w.write_with_tag(21, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Asteroid {
    pub pos: Option<Coord>,
    pub velocity: Option<Coord>,
    pub facing: Option<f32>,
    pub rvel: Option<f32>,
    pub life: Option<f32>,
}

impl<'a> MessageRead<'a> for Asteroid {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = Some(r.read_message::<Coord>(bytes)?),
                Ok(18) => msg.velocity = Some(r.read_message::<Coord>(bytes)?),
                Ok(29) => msg.facing = Some(r.read_float(bytes)?),
                Ok(37) => msg.rvel = Some(r.read_float(bytes)?),
                Ok(45) => msg.life = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Asteroid {
    fn get_size(&self) -> usize {
        0
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.velocity.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.facing.as_ref().map_or(0, |_| 1 + 4)
        + self.rvel.as_ref().map_or(0, |_| 1 + 4)
        + self.life.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.pos { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) =self.velocity { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) =self.facing { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) =self.rvel { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) =self.life { w.write_with_tag(45, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Asteroids {
    pub asteroids: Vec<Asteroid>,
}

impl<'a> MessageRead<'a> for Asteroids {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.asteroids = r.read_packed(bytes, |r, bytes| r.read_message::<Asteroid>(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Asteroids {
    fn get_size(&self) -> usize {
        0
        + if self.asteroids.is_empty() { 0 } else { 1 + sizeof_len(self.asteroids.iter().map(|s| sizeof_len((s).get_size())).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.asteroids, |w, m| w.write_message(m), &|m| sizeof_len((m).get_size()))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Join<'a> {
    pub nickname: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Join<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nickname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Join<'a> {
    fn get_size(&self) -> usize {
        0
        + self.nickname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.nickname { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinAck {
    pub id: Option<i32>,
    pub pos: Option<Coord>,
}

impl<'a> MessageRead<'a> for JoinAck {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = Some(r.read_int32(bytes)?),
                Ok(18) => msg.pos = Some(r.read_message::<Coord>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for JoinAck {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) =self.pos { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OtherJoined<'a> {
    pub id: Option<i32>,
    pub nickname: Option<Cow<'a, str>>,
    pub pos: Option<Coord>,
}

impl<'a> MessageRead<'a> for OtherJoined<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = Some(r.read_int32(bytes)?),
                Ok(18) => msg.nickname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.pos = Some(r.read_message::<Coord>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OtherJoined<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.nickname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.pos.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) =self.nickname { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) =self.pos { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Leave { }

impl<'a> MessageRead<'a> for Leave {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Leave { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OtherLeft {
    pub id: Option<i32>,
}

impl<'a> MessageRead<'a> for OtherLeft {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OtherLeft {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.id { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Spawn {
    pub entity: mod_Spawn::OneOfentity,
}

impl<'a> MessageRead<'a> for Spawn {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entity = mod_Spawn::OneOfentity::asteroids(r.read_message::<Asteroids>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Spawn {
    fn get_size(&self) -> usize {
        0
        + match self.entity {
            mod_Spawn::OneOfentity::asteroids(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Spawn::OneOfentity::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.entity {            mod_Spawn::OneOfentity::asteroids(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Spawn::OneOfentity::None => {},
    }        Ok(())
    }
}

pub mod mod_Spawn {

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    UNKNOWN = 0,
    ASTEROID = 1,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::UNKNOWN
    }
}

impl From<i32> for Kind {
    fn from(i: i32) -> Self {
        match i {
            0 => Kind::UNKNOWN,
            1 => Kind::ASTEROID,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfentity {
    asteroids(Asteroids),
    None,
}

impl Default for OneOfentity {
    fn default() -> Self {
        OneOfentity::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Heartbeat { }

impl<'a> MessageRead<'a> for Heartbeat {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Heartbeat { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message<'a> {
    pub msg: mod_Message::OneOfmsg<'a>,
}

impl<'a> MessageRead<'a> for Message<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = mod_Message::OneOfmsg::join(r.read_message::<Join>(bytes)?),
                Ok(18) => msg.msg = mod_Message::OneOfmsg::join_ack(r.read_message::<JoinAck>(bytes)?),
                Ok(26) => msg.msg = mod_Message::OneOfmsg::other_joined(r.read_message::<OtherJoined>(bytes)?),
                Ok(34) => msg.msg = mod_Message::OneOfmsg::leave(r.read_message::<Leave>(bytes)?),
                Ok(42) => msg.msg = mod_Message::OneOfmsg::other_left(r.read_message::<OtherLeft>(bytes)?),
                Ok(50) => msg.msg = mod_Message::OneOfmsg::spawn(r.read_message::<Spawn>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Message<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.msg {
            mod_Message::OneOfmsg::join(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::join_ack(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::other_joined(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::leave(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::other_left(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::spawn(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Message::OneOfmsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg {            mod_Message::OneOfmsg::join(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::join_ack(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::other_joined(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::leave(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::other_left(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::spawn(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Message::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Message {

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    UNKNOWN = 0,
    JOIN = 1,
    JOIN_ACK = 2,
    OTHER_JOINED = 3,
    LEAVE = 4,
    OTHER_LEFT = 5,
    SPAWN_ASTEROID = 6,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::UNKNOWN
    }
}

impl From<i32> for Kind {
    fn from(i: i32) -> Self {
        match i {
            0 => Kind::UNKNOWN,
            1 => Kind::JOIN,
            2 => Kind::JOIN_ACK,
            3 => Kind::OTHER_JOINED,
            4 => Kind::LEAVE,
            5 => Kind::OTHER_LEFT,
            6 => Kind::SPAWN_ASTEROID,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg<'a> {
    join(Join<'a>),
    join_ack(JoinAck),
    other_joined(OtherJoined<'a>),
    leave(Leave),
    other_left(OtherLeft),
    spawn(Spawn),
    None,
}

impl<'a> Default for OneOfmsg<'a> {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}

