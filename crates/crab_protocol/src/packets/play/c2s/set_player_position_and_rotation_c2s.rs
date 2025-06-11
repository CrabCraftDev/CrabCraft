use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;
use crate::types::position::{Position, Rotation};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "move_player_pos_rot", id = 0x1E, state = Play)]
pub struct SetPlayerPositionAndRotationC2SPacket {
    /// Position is located at player's feet
    pub position: Position,
    pub rotation: Rotation,
    pub movement_flags: PlayerMovementFlags,
}
