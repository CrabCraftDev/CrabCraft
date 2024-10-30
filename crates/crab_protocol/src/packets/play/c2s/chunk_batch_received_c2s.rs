use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x09, state = Play)]
pub struct ChunkBatchReceivedC2SPacket {
    pub chunks_per_tick: f32,
}
