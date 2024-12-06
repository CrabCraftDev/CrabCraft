use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chunk_batch_start", id = 0x0D, state = Play)]
pub struct ChunkBatchStartS2CPacket;
