use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PigVariant {}

impl Registrable for PigVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:pig_variant");
}
