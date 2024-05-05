use bytes::BytesMut;

use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol_macros::ProtocolSerializer;

#[derive(Debug, PartialEq, ProtocolSerializer)]
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
    packet.serialize(&mut buf).unwrap();
}

#[test]
fn serialize_too_big() {
    let packet = BoundedStructC2S {
        name: "123456789!".to_string(),
    };

    let mut buf = BytesMut::new();
    packet
        .serialize(&mut buf)
        .expect_err("Expected error when serializing too big string");
}
