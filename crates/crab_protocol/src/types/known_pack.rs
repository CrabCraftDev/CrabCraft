use crab_macros::ProtocolSerializer;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
