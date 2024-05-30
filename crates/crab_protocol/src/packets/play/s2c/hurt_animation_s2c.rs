use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x24, state = Play)]
pub struct HurtAnimationS2CPacket {
    pub window_id: u8,
    pub damage_direction: f32,
}
