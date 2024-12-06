use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::difficulty::Difficulty;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "change_difficulty", id = 0x03, state = Play)]
pub struct ChangeDifficultyC2SPacket {
    pub new_difficulty: Difficulty,
}
