use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

use crate::types::text::TextComponent;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x1B, state = Configuration)]
pub struct DisconnectS2CPacket {
    pub reason: TextComponent,
}
