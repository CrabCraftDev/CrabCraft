use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_title_text", id = 0x6B, state = Play)]
pub struct SetTitleTextS2CPacket {
    pub title_text: Text,
}
