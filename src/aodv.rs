extern crate tokio_core;

use std::io;
use std::net::{Ipv4Addr, SocketAddr};

use rreq::*;
use rrep::*;
use rerr::*;

use tokio_core::net::{UdpSocket, UdpCodec};

pub struct ParseError;

impl ParseError {
    pub fn new() -> io::Error {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Unable to parse bit message as AODV message",
        )
    }
}

pub enum AodvMessage {
    Rreq(RREQ),
    Rrep(RREP),
    Rerr(RERR),
    Hello(RREP),
    Ack,
}

impl AodvMessage {
    pub fn parse(b: &[u8]) -> Result<Self, io::Error> {
        if b.len() == 0 {
            return Err(ParseError::new());
        }
        // Type, Length, Multiple of 4
        match (b[0], b.len(), b.len() % 4) {
            (1, 24, 0) => Ok(AodvMessage::Rreq(RREQ::new(b)?)),
            (2, 20, 0) => Ok(AodvMessage::Rrep(RREP::new(b)?)),
            (3, _, 0) => Ok(AodvMessage::Rerr(RERR::new(b)?)),
            (4, 2, 2) => Ok(AodvMessage::Ack),
            (_, _, _) => Err(ParseError::new()),
        }
    }
    pub fn bit_message(&self) -> Vec<u8> {
        match self {
            &AodvMessage::Rreq(ref r) => r.bit_message(),
            &AodvMessage::Rrep(ref r) => r.bit_message(),
            &AodvMessage::Rerr(ref r) => r.bit_message(),
            &AodvMessage::Hello(ref r) => r.bit_message(),
            &AodvMessage::Ack => vec![4, 0],
        }
    }

    pub fn handle_message(self, addr: SocketAddr) {
        match self {
            AodvMessage::Rreq(r) => r.handle_message(),
            AodvMessage::Rrep(r) => r.handle_message(),
            AodvMessage::Rerr(r) => r.handle_message(),
            AodvMessage::Hello(r) => r.handle_message(),
            AodvMessage::Ack => println!("Received Ack from {}", addr),
        }
    }
}

pub struct AodvCodec;

impl UdpCodec for AodvCodec {
    type In = (SocketAddr, AodvMessage);
    type Out = (SocketAddr, AodvMessage);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> Result<Self::In, io::Error> {
        Ok((*addr, AodvMessage::parse(buf)?))
    }

    fn encode(&mut self, (addr, msg): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        into.extend(msg.bit_message());
        addr
    }
}
