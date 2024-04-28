use bytes::Bytes;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x0E, state = Play)]
pub struct ChunkBiomesS2CPacket {
    pub chunk_biomes_data: Vec<ChunkBiomeData>,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct ChunkBiomeData {
    pub chunk_z: i32,
    pub chunk_x: i32,
    pub biome_data: Bytes,
}
