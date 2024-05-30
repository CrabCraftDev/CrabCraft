use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x23, state = Play)]
pub struct OpenHorseScreenS2CPacket {
    pub window_id: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_count: i32,
    pub entity_id: i32,
}
