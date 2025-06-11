use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_chunk_cache_center", id = 0x57, state = Play)]
pub struct SetCenterChunkS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub x: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub z: i32,
}
