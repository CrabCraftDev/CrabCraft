use std::collections::HashMap;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "update_tags", ids(Play = 0x7F, Configuration = 0x0D))]
pub struct UpdateTagsS2CPacket {
    pub tags: HashMap<NamespacedKey, Vec<Tag>>,
}

#[derive(ProtocolSerializer, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub name: NamespacedKey,
    #[protocol_attribute(protocol_type = Vec<VarInt>)]
    pub entries: Vec<i32>,
}
