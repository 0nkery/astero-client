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
use std::collections::HashMap;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl<'a> MessageRead<'a> for Coord {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = r.read_float(bytes)?,
                Ok(21) => msg.y = r.read_float(bytes)?,
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
        + 1 + 4
        + 1 + 4
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(13, |w| w.write_float(*&self.x))?;
        w.write_with_tag(21, |w| w.write_float(*&self.y))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Asteroid {
    pub pos: Coord,
    pub velocity: Coord,
    pub facing: f32,
    pub rvel: f32,
    pub life: f32,
}

impl<'a> MessageRead<'a> for Asteroid {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = r.read_message::<Coord>(bytes)?,
                Ok(18) => msg.velocity = r.read_message::<Coord>(bytes)?,
                Ok(29) => msg.facing = r.read_float(bytes)?,
                Ok(37) => msg.rvel = r.read_float(bytes)?,
                Ok(45) => msg.life = r.read_float(bytes)?,
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
        + 1 + sizeof_len((&self.pos).get_size())
        + 1 + sizeof_len((&self.velocity).get_size())
        + 1 + 4
        + 1 + 4
        + 1 + 4
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.pos))?;
        w.write_with_tag(18, |w| w.write_message(&self.velocity))?;
        w.write_with_tag(29, |w| w.write_float(*&self.facing))?;
        w.write_with_tag(37, |w| w.write_float(*&self.rvel))?;
        w.write_with_tag(45, |w| w.write_float(*&self.life))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Asteroids {
    pub entities: HashMap<u32, Asteroid>,
}

impl<'a> MessageRead<'a> for Asteroids {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_uint32(bytes), |r, bytes| r.read_message::<Asteroid>(bytes))?;
                    msg.entities.insert(key, value);
                }
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
        + self.entities.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_varint(*(k) as u64) + sizeof_len((v).get_size()))).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.entities.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_varint(*(k) as u64) + sizeof_len((v).get_size()), 8, |w| w.write_uint32(*k), 18, |w| w.write_message(v)))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Join<'a> {
    pub nickname: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Join<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nickname = r.read_string(bytes).map(Cow::Borrowed)?,
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
        + 1 + sizeof_len((&self.nickname).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.nickname))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinAck {
    pub id: u32,
    pub pos: Coord,
}

impl<'a> MessageRead<'a> for JoinAck {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.pos = r.read_message::<Coord>(bytes)?,
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
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.pos).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_message(&self.pos))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OtherJoined<'a> {
    pub id: u32,
    pub nickname: Cow<'a, str>,
    pub pos: Coord,
}

impl<'a> MessageRead<'a> for OtherJoined<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.nickname = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.pos = r.read_message::<Coord>(bytes)?,
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
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.nickname).len())
        + 1 + sizeof_len((&self.pos).get_size())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.nickname))?;
        w.write_with_tag(26, |w| w.write_message(&self.pos))?;
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
    pub id: u32,
}

impl<'a> MessageRead<'a> for OtherLeft {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
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
        + 1 + sizeof_varint(*(&self.id) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
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
pub struct Client<'a> {
    pub msg: mod_Client::OneOfmsg<'a>,
}

impl<'a> MessageRead<'a> for Client<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = mod_Client::OneOfmsg::join(r.read_message::<Join>(bytes)?),
                Ok(18) => msg.msg = mod_Client::OneOfmsg::leave(r.read_message::<Leave>(bytes)?),
                Ok(26) => msg.msg = mod_Client::OneOfmsg::heartbeat(r.read_message::<Heartbeat>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Client<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.msg {
            mod_Client::OneOfmsg::join(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfmsg::leave(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfmsg::heartbeat(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfmsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg {            mod_Client::OneOfmsg::join(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Client::OneOfmsg::leave(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Client::OneOfmsg::heartbeat(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Client::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Client {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg<'a> {
    join(Join<'a>),
    leave(Leave),
    heartbeat(Heartbeat),
    None,
}

impl<'a> Default for OneOfmsg<'a> {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Server<'a> {
    pub msg: mod_Server::OneOfmsg<'a>,
}

impl<'a> MessageRead<'a> for Server<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = mod_Server::OneOfmsg::join_ack(r.read_message::<JoinAck>(bytes)?),
                Ok(18) => msg.msg = mod_Server::OneOfmsg::other_joined(r.read_message::<OtherJoined>(bytes)?),
                Ok(26) => msg.msg = mod_Server::OneOfmsg::other_left(r.read_message::<OtherLeft>(bytes)?),
                Ok(34) => msg.msg = mod_Server::OneOfmsg::spawn(r.read_message::<Spawn>(bytes)?),
                Ok(42) => msg.msg = mod_Server::OneOfmsg::heartbeat(r.read_message::<Heartbeat>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Server<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.msg {
            mod_Server::OneOfmsg::join_ack(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfmsg::other_joined(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfmsg::other_left(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfmsg::spawn(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfmsg::heartbeat(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfmsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.msg {            mod_Server::OneOfmsg::join_ack(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Server::OneOfmsg::other_joined(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Server::OneOfmsg::other_left(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Server::OneOfmsg::spawn(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Server::OneOfmsg::heartbeat(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Server::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Server {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg<'a> {
    join_ack(JoinAck),
    other_joined(OtherJoined<'a>),
    other_left(OtherLeft),
    spawn(Spawn),
    heartbeat(Heartbeat),
    None,
}

impl<'a> Default for OneOfmsg<'a> {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}

