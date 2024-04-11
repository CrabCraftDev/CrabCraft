use bytes::BytesMut;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::packet::Packet;
use crab_protocol::protocol::{PacketDirection, ProtocolState};
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::raw_packet::RawPacket;

#[derive(Debug, PartialEq)]
pub struct TestC2S {
    pub name: String,
}

impl Packet for TestC2S {
    const ID: i32 = 0;
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE: ProtocolState = ProtocolState::Handshake;

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(TestC2S {
            name: buf.get_string()?,
        })
    }

    fn serialize_content(&self, mut buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        self.name.serialize(&mut buf)?;
        Ok(())
    }
}

#[test]
fn decoding() {
    let mut buf = BytesMut::new();
    buf.put_string("hi!".to_string());
    let raw_packet = RawPacket::new(0, buf.freeze());

    let decoded = raw_packet.decode::<TestC2S>().unwrap();
    println!("{:?}", decoded);
}

#[test]
fn cycle() {
    let packet = TestC2S {
        name: "hi!".to_string(),
    };

    let mut buf = BytesMut::new();
    packet.serialize_content(&mut buf).unwrap();
    let raw_packet = RawPacket::new(0, buf.freeze());
    let decoded = raw_packet.decode::<TestC2S>().unwrap();

    assert_eq!(packet, decoded);
}
