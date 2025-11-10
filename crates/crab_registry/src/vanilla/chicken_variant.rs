use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChickenVariant {}

impl Registrable for ChickenVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:chicken_variant");
}
