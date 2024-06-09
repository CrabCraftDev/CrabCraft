use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use bytes::Bytes;

use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CookieResponseC2SPacket {
    pub key: Identifier,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Option<Bytes>,
}

impl Packet for CookieResponseC2SPacket {
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x01), (Login, 0x04), (Play, 0x11)];
}
