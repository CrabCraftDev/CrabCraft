use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1E, state = Play)]
pub struct SetPlayerRotationC2SPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: PlayerMovementFlags,
}
