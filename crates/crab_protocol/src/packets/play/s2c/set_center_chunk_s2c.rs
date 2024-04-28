use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x52, state = Play)]
pub struct SetCenterChunkS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub x: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub z: i32,
}
