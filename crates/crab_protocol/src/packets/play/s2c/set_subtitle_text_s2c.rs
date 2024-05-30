use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x63, state = Play)]
pub struct SetSubtitleTextS2CPacket {
    pub subtitle_text: TextComponent,
}
