use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x0D, state = Play)]
pub struct ChunkBatchStartS2CPacket;
