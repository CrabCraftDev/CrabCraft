use std::collections::HashMap;

use crab_protocol_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;
use crate::types::varint::VarInt;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct UpdateTagsS2CPacket {
    pub tags: HashMap<Identifier, Vec<Tag>>,
}

#[derive(ProtocolSerializer, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub name: Identifier,
    #[protocol_attribute(protocol_type = Vec<VarInt>)]
    pub entries: Vec<i32>,
}

impl Packet for UpdateTagsS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x78), (Configuration, 0x0D)];
}
