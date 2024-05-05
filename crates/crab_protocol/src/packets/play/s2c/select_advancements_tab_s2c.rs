use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x48, state = Play)]
pub struct SelectAdvancementsTabS2CPacket {
    pub identifier: Option<Identifier>,
}
