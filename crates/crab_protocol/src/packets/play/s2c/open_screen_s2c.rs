use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "open_screen", id = 0x34, state = Play)]
pub struct OpenScreenS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_type: i32,
    pub title: Text,
}
