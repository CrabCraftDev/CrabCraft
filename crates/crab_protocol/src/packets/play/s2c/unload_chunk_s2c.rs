use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x1F, state = Play)]
pub struct UnloadChunkS2CPacket {
    pub chunk_z: i32,
    pub chunk_x: i32,
}
