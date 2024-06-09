use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CookieRequestS2CPacket {
    pub key: Identifier,
}

impl Packet for CookieRequestS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x00), (Login, 0x05), (Play, 0x16)];
}
