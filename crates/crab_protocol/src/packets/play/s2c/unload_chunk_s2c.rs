use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "forget_level_chunk", id = 0x21, state = Play)]
pub struct UnloadChunkS2CPacket {
    pub chunk_z: i32,
    pub chunk_x: i32,
}
