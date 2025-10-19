use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::SimpleRegistry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PigVariant {}

impl Registrable for PigVariant {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:pig_variant");
}
