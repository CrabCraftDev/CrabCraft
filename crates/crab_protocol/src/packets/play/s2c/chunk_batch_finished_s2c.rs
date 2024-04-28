use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x0C, state = Play)]
pub struct ChunkBatchFinishedS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub number_of_chunks: i32,
}
