use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x69, state = Play)]
pub struct SystemChatMessageS2CPacket {
    pub content: TextComponent,
    pub is_actionbar: bool,
}
