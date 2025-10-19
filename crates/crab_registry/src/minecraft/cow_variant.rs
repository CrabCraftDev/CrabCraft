use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::SimpleRegistry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CowVariant {}

impl Registrable for CowVariant {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:cow_variant");
}
