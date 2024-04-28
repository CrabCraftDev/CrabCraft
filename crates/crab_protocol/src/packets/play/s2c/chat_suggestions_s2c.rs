use strum::FromRepr;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x17, state = Play)]
pub struct ChatSuggestionsS2CPacket {
    pub action: ChatSuggestionAction,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum ChatSuggestionAction {
    Add,
    Remove,
    Set,
}
