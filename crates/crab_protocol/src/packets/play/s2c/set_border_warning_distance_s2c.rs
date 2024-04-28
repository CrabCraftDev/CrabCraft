use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x4F, state = Play)]
pub struct SetBorderWarningDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_blocks: i32,
}
