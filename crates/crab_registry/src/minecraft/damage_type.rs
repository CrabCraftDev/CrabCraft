use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::SimpleRegistry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DamageType {}

impl Registrable for DamageType {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:damage_type");
}
