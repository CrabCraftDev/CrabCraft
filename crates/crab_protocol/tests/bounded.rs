use bytes::BytesMut;

use crab_protocol::packet::Packet;
use crab_protocol::protocol::ProtocolState::Handshake;
use crab_protocol_macros::Packet;

#[derive(Debug, PartialEq, Packet)]
#[packet(id = 0, state = Handshake)]
pub struct BoundedStructC2S {
    #[protocol_attribute(bounded = 5)]
    pub name: String,
}

#[test]
fn serialize_normally() {
    let packet = BoundedStructC2S {
        name: "hi 💖".to_string(),
    };

    let mut buf = BytesMut::new();
    packet.serialize_content(&mut buf).unwrap();
}

#[test]
fn serialize_too_big() {
    let packet = BoundedStructC2S {
        name: "123456789!".to_string(),
    };

    let mut buf = BytesMut::new();
    packet
        .serialize_content(&mut buf)
        .expect_err("Expected error when serializing too big string");
}
