use bytes::Bytes;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bit_set::BitSet;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::types::varint::VarInt;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x28, state = Play)]
pub struct UpdateLightS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub chunk_x: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chunk_z: i32,
    pub light_data: LightData,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct LightData {
    pub sky_light_map: BitSet,
    pub block_light_map: BitSet,
    pub empty_sky_light_map: BitSet,
    pub empty_block_light_map: BitSet,
    pub sky_light_array: Vec<Bytes>,
    pub block_light_array: Vec<Bytes>,
}
