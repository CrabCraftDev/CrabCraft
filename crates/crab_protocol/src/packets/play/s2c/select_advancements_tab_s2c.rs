use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x48, state = Play)]
pub struct SelectAdvancementsTabS2CPacket {
    pub identifier: Option<Identifier>,
}
