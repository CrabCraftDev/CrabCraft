use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "edit_book", id = 0x17, state = Play)]
pub struct EditBookC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub slot: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub count: i32,
    #[protocol_attribute(bounded = 100)]
    pub entries: Vec<String>,
    #[protocol_attribute(bounded = 32)]
    pub title: Option<String>,
}
