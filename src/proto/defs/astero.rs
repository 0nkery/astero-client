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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Entity {
    UNKNOWN_ENTITY = 0,
    ASTEROID = 1,
    PLAYER = 2,
}

impl Default for Entity {
    fn default() -> Self {
        Entity::UNKNOWN_ENTITY
    }
}

impl From<i32> for Entity {
    fn from(i: i32) -> Self {
        match i {
            0 => Entity::UNKNOWN_ENTITY,
            1 => Entity::ASTEROID,
            2 => Entity::PLAYER,
            _ => Self::default(),
        }
    }
}

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
pub struct Body {
    pub pos: Coord,
    pub vel: Coord,
    pub rot: Option<f32>,
    pub rvel: Option<f32>,
    pub size: Option<f32>,
}

impl<'a> MessageRead<'a> for Body {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pos = r.read_message::<Coord>(bytes)?,
                Ok(18) => msg.vel = r.read_message::<Coord>(bytes)?,
                Ok(29) => msg.rot = Some(r.read_float(bytes)?),
                Ok(37) => msg.rvel = Some(r.read_float(bytes)?),
                Ok(45) => msg.size = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Body {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.pos).get_size())
        + 1 + sizeof_len((&self.vel).get_size())
        + self.rot.as_ref().map_or(0, |_| 1 + 4)
        + self.rvel.as_ref().map_or(0, |_| 1 + 4)
        + self.size.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.pos))?;
        w.write_with_tag(18, |w| w.write_message(&self.vel))?;
        if let Some(ref s) =self.rot { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) =self.rvel { w.write_with_tag(37, |w| w.write_float(*s))?; }
        if let Some(ref s) =self.size { w.write_with_tag(45, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Asteroid {
    pub id: u32,
    pub body: Body,
    pub life: Option<f32>,
}

impl<'a> MessageRead<'a> for Asteroid {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.body = r.read_message::<Body>(bytes)?,
                Ok(29) => msg.life = Some(r.read_float(bytes)?),
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
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.body).get_size())
        + self.life.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_message(&self.body))?;
        if let Some(ref s) =self.life { w.write_with_tag(29, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Shot {
    pub body: Body,
    pub ttl: f32,
}

impl<'a> MessageRead<'a> for Shot {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.body = r.read_message::<Body>(bytes)?,
                Ok(21) => msg.ttl = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Shot {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.body).get_size())
        + 1 + 4
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.body))?;
        w.write_with_tag(21, |w| w.write_float(*&self.ttl))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Player<'a> {
    pub id: u32,
    pub body: Body,
    pub nickname: Option<Cow<'a, str>>,
    pub life: Option<f32>,
}

impl<'a> MessageRead<'a> for Player<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.body = r.read_message::<Body>(bytes)?,
                Ok(26) => msg.nickname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(37) => msg.life = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Player<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.body).get_size())
        + self.nickname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.life.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_message(&self.body))?;
        if let Some(ref s) =self.nickname { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) =self.life { w.write_with_tag(37, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Create<'a> {
    pub Entity: mod_Create::OneOfEntity<'a>,
}

impl<'a> MessageRead<'a> for Create<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Entity = mod_Create::OneOfEntity::player(r.read_message::<Player>(bytes)?),
                Ok(18) => msg.Entity = mod_Create::OneOfEntity::asteroid(r.read_message::<Asteroid>(bytes)?),
                Ok(26) => msg.Entity = mod_Create::OneOfEntity::shot(r.read_message::<Shot>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Create<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.Entity {
            mod_Create::OneOfEntity::player(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Create::OneOfEntity::asteroid(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Create::OneOfEntity::shot(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Create::OneOfEntity::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Entity {            mod_Create::OneOfEntity::player(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Create::OneOfEntity::asteroid(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Create::OneOfEntity::shot(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Create::OneOfEntity::None => {},
    }        Ok(())
    }
}

pub mod mod_Create {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfEntity<'a> {
    player(Player<'a>),
    asteroid(Asteroid),
    shot(Shot),
    None,
}

impl<'a> Default for OneOfEntity<'a> {
    fn default() -> Self {
        OneOfEntity::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Destroy {
    pub id: u32,
    pub entity: Entity,
}

impl<'a> MessageRead<'a> for Destroy {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(16) => msg.entity = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Destroy {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_varint(*(&self.entity) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.entity as i32))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub Entity: mod_Update::OneOfEntity<'a>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Entity = mod_Update::OneOfEntity::player(r.read_message::<Player>(bytes)?),
                Ok(18) => msg.Entity = mod_Update::OneOfEntity::asteroid(r.read_message::<Asteroid>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.Entity {
            mod_Update::OneOfEntity::player(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Update::OneOfEntity::asteroid(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Update::OneOfEntity::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Entity {            mod_Update::OneOfEntity::player(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Update::OneOfEntity::asteroid(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Update::OneOfEntity::None => {},
    }        Ok(())
    }
}

pub mod mod_Update {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfEntity<'a> {
    player(Player<'a>),
    asteroid(Asteroid),
    None,
}

impl<'a> Default for OneOfEntity<'a> {
    fn default() -> Self {
        OneOfEntity::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ManyUpdates<'a> {
    pub updates: Vec<Update<'a>>,
}

impl<'a> MessageRead<'a> for ManyUpdates<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.updates.push(r.read_message::<Update>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ManyUpdates<'a> {
    fn get_size(&self) -> usize {
        0
        + self.updates.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.updates { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinPayload<'a> {
    pub nickname: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for JoinPayload<'a> {
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

impl<'a> MessageWrite for JoinPayload<'a> {
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
pub struct Input {
    pub turn: Option<i32>,
    pub accel: Option<i32>,
    pub fire: Option<bool>,
}

impl<'a> MessageRead<'a> for Input {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.turn = Some(r.read_sint32(bytes)?),
                Ok(16) => msg.accel = Some(r.read_sint32(bytes)?),
                Ok(24) => msg.fire = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Input {
    fn get_size(&self) -> usize {
        0
        + self.turn.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.accel.as_ref().map_or(0, |m| 1 + sizeof_sint32(*(m)))
        + self.fire.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.turn { w.write_with_tag(8, |w| w.write_sint32(*s))?; }
        if let Some(ref s) =self.accel { w.write_with_tag(16, |w| w.write_sint32(*s))?; }
        if let Some(ref s) =self.fire { w.write_with_tag(24, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Client {
    pub Msg: mod_Client::OneOfMsg,
}

impl<'a> MessageRead<'a> for Client {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Msg = mod_Client::OneOfMsg::input(r.read_message::<Input>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Client {
    fn get_size(&self) -> usize {
        0
        + match self.Msg {
            mod_Client::OneOfMsg::input(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Msg {            mod_Client::OneOfMsg::input(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Client {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfMsg {
    input(Input),
    None,
}

impl Default for OneOfMsg {
    fn default() -> Self {
        OneOfMsg::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Server<'a> {
    pub Msg: mod_Server::OneOfMsg<'a>,
}

impl<'a> MessageRead<'a> for Server<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Msg = mod_Server::OneOfMsg::create(r.read_message::<Create>(bytes)?),
                Ok(18) => msg.Msg = mod_Server::OneOfMsg::destroy(r.read_message::<Destroy>(bytes)?),
                Ok(26) => msg.Msg = mod_Server::OneOfMsg::updates(r.read_message::<ManyUpdates>(bytes)?),
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
        + match self.Msg {
            mod_Server::OneOfMsg::create(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::destroy(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::updates(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Msg {            mod_Server::OneOfMsg::create(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::destroy(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::updates(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Server {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfMsg<'a> {
    create(Create<'a>),
    destroy(Destroy),
    updates(ManyUpdates<'a>),
    None,
}

impl<'a> Default for OneOfMsg<'a> {
    fn default() -> Self {
        OneOfMsg::None
    }
}

}

