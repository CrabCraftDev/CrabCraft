use crab_protocol_macros::ProtocolSerializer;

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
