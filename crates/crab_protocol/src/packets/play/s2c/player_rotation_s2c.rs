use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_rotation", id = 0x43, state = Play)]
pub struct PlayerRotationS2CPacket {
    pub yaw: f32,
    pub pitch: f32,
}
