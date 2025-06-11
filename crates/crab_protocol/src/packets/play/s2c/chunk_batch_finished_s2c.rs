use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chunk_batch_finished", id = 0x0B, state = Play)]
pub struct ChunkBatchFinishedS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub number_of_chunks: i32,
}
