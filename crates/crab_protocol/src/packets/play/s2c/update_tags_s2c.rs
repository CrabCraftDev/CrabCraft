use std::collections::HashMap;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::tag::Tag;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x74, state = Configuration)]
pub struct UpdateTagsS2CPacket {
    pub tags: HashMap<Identifier, Vec<Tag>>,
}
