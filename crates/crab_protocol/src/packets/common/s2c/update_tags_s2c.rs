use std::collections::HashMap;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct UpdateTagsS2CPacket {
    pub tags: HashMap<NamespacedKey, Vec<Tag>>,
}

#[derive(ProtocolSerializer, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub name: NamespacedKey,
    #[protocol_attribute(protocol_type = Vec<VarInt>)]
    pub entries: Vec<i32>,
}

impl Packet for UpdateTagsS2CPacket {
    const NAME: &'static str = "update_tags";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x7F), (Configuration, 0x0D)];
}
