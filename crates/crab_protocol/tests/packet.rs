use bytes::{Bytes, BytesMut};

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::packet::Packet;
use crab_protocol::protocol::{PacketDirection, ProtocolState};
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::raw_packet::RawPacket;

#[derive(Debug, PartialEq)]
pub struct TestC2S {
    pub name: String,
}

impl Packet for TestC2S {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(ProtocolState::Handshake, 0)];
}

impl ProtocolSerializer for TestC2S {
    fn serialize(&self, mut buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        self.name.serialize(&mut buf)?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(TestC2S {
            name: buf.get_string()?,
        })
    }
}

#[test]
fn decoding_correct_state() {
    let mut buf = BytesMut::new();
    buf.put_string("hi!");
    let raw_packet = RawPacket::new(0, buf.freeze());

    raw_packet.decode::<TestC2S>(ProtocolState::Handshake).unwrap();
}

#[test]
fn decoding_invalid_state() {
    let raw_packet = RawPacket::new(0, Bytes::from("\0x03hi!"));
    assert!(raw_packet.decode::<TestC2S>(ProtocolState::Status).is_none());
}

#[test]
fn cycle() {
    let packet = TestC2S {
        name: "hi!".to_string(),
    };

    let mut buf = BytesMut::new();
    packet.serialize(&mut buf).unwrap();
    let raw_packet = RawPacket::new(0, buf.freeze());
    let decoded = raw_packet.decode::<TestC2S>(ProtocolState::Handshake).unwrap();

    assert_eq!(packet, decoded);
}
