use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;
use crate::types::position::Position;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct SetPlayerPositionC2SPacket {
    /// Position is located at player's feet
    pub position: Position,
    pub flags: PlayerMovementFlags,
}
