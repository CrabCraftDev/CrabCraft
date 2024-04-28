use crab_protocol_macros::ProtocolType;

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
