use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x22, state = Play)]
pub struct GameEventS2CPacket {
    pub event: u8,
    pub value: f32,
}
