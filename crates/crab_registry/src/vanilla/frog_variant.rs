use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrogVariant {}

impl Registrable for FrogVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:frog_variant");
}
