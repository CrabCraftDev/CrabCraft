use bytes::{BufMut, BytesMut};
use strum::FromRepr;

use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol_macros::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

#[test]
fn serialize_enum() {
    let mut buf = Vec::new();
    NextState::serialize(&NextState::Status, &mut buf).unwrap();
    assert_eq!(buf, &[0x01]);
}

#[test]
fn deserialize_enum() {
    let mut buf = BytesMut::new();
    buf.put_u8(0x02);
    let state = NextState::deserialize(&mut buf).unwrap();
    assert_eq!(state, NextState::Login);
}

#[test]
fn deserialize_enum_invalid() {
    let mut buf = BytesMut::new();
    buf.put_u8(0x03);
    let state = NextState::deserialize(&mut buf);
    assert!(state.is_err());
}
