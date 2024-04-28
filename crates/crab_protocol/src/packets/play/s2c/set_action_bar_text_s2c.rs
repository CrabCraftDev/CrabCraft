use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x4A, state = Play)]
pub struct SetActionBarTextS2CPacket {
    pub action_bar_text: TextComponent,
}
