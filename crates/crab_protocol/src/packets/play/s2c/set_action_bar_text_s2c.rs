use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_action_bar_text", id = 0x51, state = Play)]
pub struct SetActionBarTextS2CPacket {
    pub action_bar_text: TextComponent,
}
