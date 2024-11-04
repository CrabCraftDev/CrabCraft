use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;
use crate::types::position::{Position, Rotation};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1D, state = Play)]
pub struct SetPlayerPositionAndRotationC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub rotation: Rotation,
    pub flags: PlayerMovementFlags,
}
