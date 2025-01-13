use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_subtitle_text", id = 0x6A, state = Play)]
pub struct SetSubtitleTextS2CPacket {
    pub subtitle_text: Text,
}
