use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "lock_difficulty", id = 0x1C, state = Play)]
pub struct LockDifficultyC2SPacket {
    pub is_locked: bool,
}
