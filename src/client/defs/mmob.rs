//! Automatically generated rust module for 'mmob.proto' file

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
pub struct JoinGame<'a> {
    pub payload: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for JoinGame<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.payload = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for JoinGame<'a> {
    fn get_size(&self) -> usize {
        0
        + self.payload.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.payload { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinAck<'a> {
    pub payload: Option<Cow<'a, [u8]>>,
}

impl<'a> MessageRead<'a> for JoinAck<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.payload = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for JoinAck<'a> {
    fn get_size(&self) -> usize {
        0
        + self.payload.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) =self.payload { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LeaveGame { }

impl<'a> MessageRead<'a> for LeaveGame {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LeaveGame { }

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
pub struct LatencyMeasure {
    pub timestamp: u64,
}

impl<'a> MessageRead<'a> for LatencyMeasure {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.timestamp = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LatencyMeasure {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.timestamp) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint64(*&self.timestamp))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Proxied<'a> {
    pub msg: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Proxied<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Proxied<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.msg).len())
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_bytes(&**&self.msg))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Client<'a> {
    pub Msg: mod_Client::OneOfMsg<'a>,
}

impl<'a> MessageRead<'a> for Client<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.Msg = mod_Client::OneOfMsg::join(r.read_message::<JoinGame>(bytes)?),
                Ok(18) => msg.Msg = mod_Client::OneOfMsg::leave(r.read_message::<LeaveGame>(bytes)?),
                Ok(26) => msg.Msg = mod_Client::OneOfMsg::heartbeat(r.read_message::<Heartbeat>(bytes)?),
                Ok(34) => msg.Msg = mod_Client::OneOfMsg::latency_measure(r.read_message::<LatencyMeasure>(bytes)?),
                Ok(42) => msg.Msg = mod_Client::OneOfMsg::proxied(r.read_message::<Proxied>(bytes)?),
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
        + match self.Msg {
            mod_Client::OneOfMsg::join(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::leave(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::heartbeat(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::latency_measure(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::proxied(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Client::OneOfMsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Msg {            mod_Client::OneOfMsg::join(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::leave(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::heartbeat(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::latency_measure(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::proxied(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Client::OneOfMsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Client {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfMsg<'a> {
    join(JoinGame<'a>),
    leave(LeaveGame),
    heartbeat(Heartbeat),
    latency_measure(LatencyMeasure),
    proxied(Proxied<'a>),
    None,
}

impl<'a> Default for OneOfMsg<'a> {
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
                Ok(10) => msg.Msg = mod_Server::OneOfMsg::join_ack(r.read_message::<JoinAck>(bytes)?),
                Ok(18) => msg.Msg = mod_Server::OneOfMsg::heartbeat(r.read_message::<Heartbeat>(bytes)?),
                Ok(26) => msg.Msg = mod_Server::OneOfMsg::latency_measure(r.read_message::<LatencyMeasure>(bytes)?),
                Ok(34) => msg.Msg = mod_Server::OneOfMsg::proxied(r.read_message::<Proxied>(bytes)?),
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
            mod_Server::OneOfMsg::join_ack(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::heartbeat(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::latency_measure(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::proxied(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Server::OneOfMsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Msg {            mod_Server::OneOfMsg::join_ack(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::heartbeat(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::latency_measure(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::proxied(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Server::OneOfMsg::None => {},
    }        Ok(())
    }
}

pub mod mod_Server {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfMsg<'a> {
    join_ack(JoinAck<'a>),
    heartbeat(Heartbeat),
    latency_measure(LatencyMeasure),
    proxied(Proxied<'a>),
    None,
}

impl<'a> Default for OneOfMsg<'a> {
    fn default() -> Self {
        OneOfMsg::None
    }
}

}

