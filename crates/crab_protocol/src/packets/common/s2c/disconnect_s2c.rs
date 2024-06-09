use crab_protocol_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct DisconnectS2CPacket {
    pub reason: TextComponent,
}

impl Packet for DisconnectS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x02), (Play, 0x1D)];
}
