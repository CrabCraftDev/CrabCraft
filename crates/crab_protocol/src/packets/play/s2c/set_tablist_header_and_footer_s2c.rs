use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x6A, state = Play)]
pub struct SetTablistHeaderAndFooterS2CPacket {
    pub header: TextComponent,
    pub footer: TextComponent,
}
