use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_border_warning_delay", id = 0x54, state = Play)]
pub struct SetBorderWarningDelayS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_time: i32,
}
