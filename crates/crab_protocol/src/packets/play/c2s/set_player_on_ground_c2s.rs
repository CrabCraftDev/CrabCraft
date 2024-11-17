use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_movement_flags::PlayerMovementFlags;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1F, state = Play)]
pub struct SetPlayerOnGroundC2SPacket {
    pub movement_flags: PlayerMovementFlags,
}
