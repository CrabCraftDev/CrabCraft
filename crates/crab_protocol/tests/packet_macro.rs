use bytes::BytesMut;

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::protocol_state::ProtocolState::Handshake;
use crab_protocol::raw_packet::RawPacket;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolSerializer};

#[derive(Debug, PartialEq, Packet, ProtocolSerializer)]
#[packet(id = 0, state = Handshake)]
pub struct TestC2S {
    pub name: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub varint: i32,
}

#[test]
fn decoding_macro() {
    let mut buf = BytesMut::new();
    buf.put_string("hi!");
    buf.put_var_int(2);
    let raw_packet = RawPacket::new(0, buf.freeze());

    let decoded = raw_packet.decode::<TestC2S>(Handshake).unwrap();
    assert_eq!(
        decoded,
        TestC2S {
            name: "hi!".to_string(),
            varint: 2,
        }
    );
}

#[test]
fn cycle_macro() {
    let packet = TestC2S {
        name: "hi!".to_string(),
        varint: 5,
    };

    let mut buf = BytesMut::new();
    packet.serialize(&mut buf).unwrap();
    let raw_packet = RawPacket::new(0, buf.freeze());
    let decoded = raw_packet.decode::<TestC2S>(Handshake).unwrap();

    assert_eq!(packet, decoded);
}
