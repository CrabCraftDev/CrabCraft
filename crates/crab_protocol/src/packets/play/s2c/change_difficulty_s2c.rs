use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::difficulty::Difficulty;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0B, state = Play)]
pub struct ChangeDifficultyS2CPacket {
    pub position: Difficulty,
    pub is_locked: bool,
}
