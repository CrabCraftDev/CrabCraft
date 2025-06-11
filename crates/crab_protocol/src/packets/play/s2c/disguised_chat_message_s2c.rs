use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "disguised_chat", id = 0x1D, state = Play)]
pub struct DisguisedChatMessageS2CPacket {
    pub message: Text,
    #[protocol_attribute(protocol_type = VarInt)]
    pub chat_type: i32,
    pub sender_name: Text,
    pub target_name: Option<Text>,
}
