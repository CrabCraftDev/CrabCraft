use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x18, state = Play)]
pub struct ChatSuggestionsS2CPacket {
    pub action: ChatSuggestionAction,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum ChatSuggestionAction {
    Add,
    Remove,
    Set,
}
