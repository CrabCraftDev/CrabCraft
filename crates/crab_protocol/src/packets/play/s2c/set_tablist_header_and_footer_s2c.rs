use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "tab_list", id = 0x74, state = Play)]
pub struct SetTablistHeaderAndFooterS2CPacket {
    pub header: Text,
    pub footer: Text,
}
