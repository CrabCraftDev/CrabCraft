use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x4A, state = Play)]
pub struct SetActionBarTextS2CPacket {
    pub action_bar_text: TextComponent,
}
