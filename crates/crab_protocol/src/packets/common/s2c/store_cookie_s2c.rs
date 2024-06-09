use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use bytes::Bytes;

use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct StoreCookieS2CPacket {
    pub key: Identifier,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Bytes,
}

impl Packet for StoreCookieS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x6B), (Configuration, 0x0A)];
}
