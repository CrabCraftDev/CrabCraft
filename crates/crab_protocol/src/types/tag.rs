use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::ProtocolType;

#[derive(ProtocolType, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub name: Identifier,
    pub entries: Vec<VarInt>,
}
