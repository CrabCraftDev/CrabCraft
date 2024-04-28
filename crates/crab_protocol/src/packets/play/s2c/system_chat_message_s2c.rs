use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x69, state = Play)]
pub struct SystemChatMessageS2CPacket {
    pub content: TextComponent,
    pub is_actionbar: bool,
}
