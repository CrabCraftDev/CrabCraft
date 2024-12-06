use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "ticking_state", id = 0x78, state = Play)]
pub struct SetTickingStateS2CPacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
