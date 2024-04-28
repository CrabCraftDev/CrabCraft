use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::difficulty::Difficulty;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x02, state = Play)]
pub struct ChangeDifficultyC2SPacket {
    pub new_difficulty: Difficulty,
}
