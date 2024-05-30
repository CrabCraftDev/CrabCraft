use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x71, state = Play)]
pub struct SetTickingStateS2CPacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
