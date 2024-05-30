use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x33, state = Play)]
pub struct OpenScreenS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_type: i32,
    pub title: TextComponent,
}
