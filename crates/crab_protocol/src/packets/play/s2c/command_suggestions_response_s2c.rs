use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::TextComponent;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x10, state = Play)]
pub struct CommandSuggestionsResponseS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub transaction_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub start_of_text_to_replace: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub length_of_text_to_replace: i32,
    pub command_suggestions: Vec<CommandSuggestion>,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct CommandSuggestion {
    pub suggestion: String,
    pub tooltip: Option<TextComponent>,
}
