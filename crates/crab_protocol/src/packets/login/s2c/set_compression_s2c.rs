use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "login_compression", id = 0x03, state = Login)]
pub struct SetCompressionS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub threshold: i32,
}
