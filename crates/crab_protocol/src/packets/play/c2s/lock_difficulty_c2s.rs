use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x16, state = Play)]
pub struct LockDifficultyC2SPacket {
    pub is_locked: bool,
}
