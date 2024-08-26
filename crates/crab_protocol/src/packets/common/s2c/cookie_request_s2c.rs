use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct CookieRequestS2CPacket {
    pub key: Identifier,
}

impl Packet for CookieRequestS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x00), (Login, 0x05), (Play, 0x16)];
}
