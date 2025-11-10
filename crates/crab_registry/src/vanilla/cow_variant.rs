use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CowVariant {}

impl Registrable for CowVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:cow_variant");
}
