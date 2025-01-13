use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "system_chat", id = 0x73, state = Play)]
pub struct SystemChatMessageS2CPacket {
    pub content: Text,
    pub is_actionbar: bool,
}
