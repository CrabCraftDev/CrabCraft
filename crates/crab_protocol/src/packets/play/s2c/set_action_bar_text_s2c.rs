use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_action_bar_text", id = 0x50, state = Play)]
pub struct SetActionBarTextS2CPacket {
    pub action_bar_text: Text,
}
