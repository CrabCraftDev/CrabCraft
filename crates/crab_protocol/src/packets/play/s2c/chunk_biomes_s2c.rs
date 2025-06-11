use bytes::Bytes;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "chunks_biomes", id = 0x0D, state = Play)]
pub struct ChunkBiomesS2CPacket {
    pub chunk_biomes_data: Vec<ChunkBiomeData>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct ChunkBiomeData {
    pub chunk_z: i32,
    pub chunk_x: i32,
    pub biome_data: Bytes,
}
