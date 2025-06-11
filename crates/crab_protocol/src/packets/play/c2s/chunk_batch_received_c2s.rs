use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chunk_batch_received", id = 0x0A, state = Play)]
pub struct ChunkBatchReceivedC2SPacket {
    pub chunks_per_tick: f32,
}
