use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x14, state = Play)]
pub struct EditBookC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub count: i32,
    #[protocol_attribute(bounded = 200)]
    pub entries: Vec<String>,
    #[protocol_attribute(bounded = 128)]
    pub title: Option<String>,
}
