use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4F, state = Play)]
pub struct SetBorderWarningDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_blocks: i32,
}
