use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "hurt_animation", id = 0x24, state = Play)]
pub struct HurtAnimationS2CPacket {
    pub entity_id: u8,
    pub damage_direction: f32,
}
