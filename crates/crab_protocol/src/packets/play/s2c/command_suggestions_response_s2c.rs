use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "command_suggestions", id = 0x10, state = Play)]
pub struct CommandSuggestionsResponseS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub start_of_text_to_replace: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub length_of_text_to_replace: i32,
    pub command_suggestions: Vec<CommandSuggestion>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct CommandSuggestion {
    pub suggestion: String,
    pub tooltip: Option<TextComponent>,
}
