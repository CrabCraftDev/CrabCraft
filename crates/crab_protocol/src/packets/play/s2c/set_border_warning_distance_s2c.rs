use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_border_warning_distance", id = 0x56, state = Play)]
pub struct SetBorderWarningDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub warning_blocks: i32,
}
