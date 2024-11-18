use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;
use crate::types::position::Rotation;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1E, state = Play)]
pub struct SetPlayerRotationC2SPacket {
    pub rotation: Rotation,
    pub movement_flags: PlayerMovementFlags,
}
