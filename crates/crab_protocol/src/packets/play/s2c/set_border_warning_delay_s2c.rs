use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x50, state = Play)]
pub struct SetBorderWarningDelayS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_time: i32,
}
