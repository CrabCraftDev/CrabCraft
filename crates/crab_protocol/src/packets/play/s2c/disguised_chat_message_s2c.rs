use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "disguised_chat", id = 0x1E, state = Play)]
pub struct DisguisedChatMessageS2CPacket {
    pub message: TextComponent,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chat_type: i32,
    pub sender_name: TextComponent,
    pub target_name: Option<TextComponent>,
}
