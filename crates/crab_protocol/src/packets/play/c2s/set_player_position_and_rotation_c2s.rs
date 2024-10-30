use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1D, state = Play)]
pub struct SetPlayerPositionAndRotationC2SPacket {
    /// Y is feet position
    pub position: Position,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: PlayerMovementFlags,
}
