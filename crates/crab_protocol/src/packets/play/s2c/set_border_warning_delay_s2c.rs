use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x4E, state = Play)]
pub struct SetBorderWarningDelayS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_time: i32,
}
