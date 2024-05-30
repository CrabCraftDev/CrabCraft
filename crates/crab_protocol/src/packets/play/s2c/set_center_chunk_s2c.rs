use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x54, state = Play)]
pub struct SetCenterChunkS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub x: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub z: i32,
}
