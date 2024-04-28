use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x63, state = Play)]
pub struct SetTitleTextS2CPacket {
    pub title_text: TextComponent,
}
