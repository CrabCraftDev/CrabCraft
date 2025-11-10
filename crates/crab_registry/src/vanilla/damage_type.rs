use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DamageType {}

impl Registrable for DamageType {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:damage_type");
}
