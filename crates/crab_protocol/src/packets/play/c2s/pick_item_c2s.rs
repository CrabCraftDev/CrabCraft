use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x20, state = Play)]
pub struct PickItemC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_to_use: i32,
}
