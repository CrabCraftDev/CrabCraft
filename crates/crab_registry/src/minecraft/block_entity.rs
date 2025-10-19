use crate::registry_key::RegistryKey;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BlockEntity {
    pub name: String,
}

impl BlockEntity {
    pub const REGISTRY: RegistryKey<Self> = RegistryKey::new("crabcraft:block_entity");
}
