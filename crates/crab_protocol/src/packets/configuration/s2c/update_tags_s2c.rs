use std::collections::HashMap;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;
use crate::types::tag::Tag;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x09, state = Configuration)]
pub struct UpdateTagsS2CPacket {
    pub tags: HashMap<Identifier, Vec<Tag>>,
}
