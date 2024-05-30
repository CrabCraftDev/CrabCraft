use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x08, state = Play)]
pub struct ChunkBatchReceivedC2SPacket {
    pub chunks_per_tick: f32,
}
