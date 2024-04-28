use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x61, state = Play)]
pub struct SetSubtitleTextS2CPacket {
    pub subtitle_text: TextComponent,
}
