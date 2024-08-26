use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x6C, state = Play)]
pub struct SystemChatMessageS2CPacket {
    pub content: TextComponent,
    pub is_actionbar: bool,
}
