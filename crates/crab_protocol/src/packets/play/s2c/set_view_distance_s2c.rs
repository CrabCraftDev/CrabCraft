use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_chunk_cache_radius", id = 0x59, state = Play)]
pub struct SetViewDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub view_distance: i32,
}
