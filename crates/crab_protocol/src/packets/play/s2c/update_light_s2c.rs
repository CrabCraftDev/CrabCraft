use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::bit_set::BitSet;
use crate::types::varint::VarInt;

#[derive(Debug, Packet, ProtocolSerializer, Clone)]
#[packet(id = 0x2A, state = Play)]
pub struct UpdateLightS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub chunk_x: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chunk_z: i32,
    pub light_data: LightData,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct LightData {
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_array: Vec<Bytes>,
    pub block_light_array: Vec<Bytes>,
}
