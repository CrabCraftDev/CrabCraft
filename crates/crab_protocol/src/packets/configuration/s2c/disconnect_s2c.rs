use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x02, state = Configuration)]
pub struct DisconnectS2CPacket {
    pub reason: TextComponent,
}
