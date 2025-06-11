use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::difficulty::Difficulty;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "change_difficulty", id = 0x0A, state = Play)]
pub struct ChangeDifficultyS2CPacket {
    pub position: Difficulty,
    pub is_locked: bool,
}
