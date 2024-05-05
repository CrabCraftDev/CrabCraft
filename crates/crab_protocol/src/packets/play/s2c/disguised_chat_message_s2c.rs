use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1C, state = Play)]
pub struct DisguisedChatMessageS2CPacket {
    pub message: TextComponent,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chat_type: i32,
    pub sender_name: TextComponent,
    pub target_name: Option<TextComponent>,
}
