use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x74, state = Play)]
pub struct SetTablistHeaderAndFooterS2CPacket {
    pub header: TextComponent,
    pub footer: TextComponent,
}
