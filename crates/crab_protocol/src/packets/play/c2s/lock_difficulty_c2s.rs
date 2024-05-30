use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x19, state = Play)]
pub struct LockDifficultyC2SPacket {
    pub is_locked: bool,
}
