use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::difficulty::Difficulty;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x0B, state = Play)]
pub struct ChangeDifficultyS2CPacket {
    pub position: Difficulty,
    pub is_locked: bool,
}
