use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x21, state = Play)]
pub struct OpenHorseScreenS2CPacket {
    pub window_id: u8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot_count: i32,
    pub entity_id: i32,
}
