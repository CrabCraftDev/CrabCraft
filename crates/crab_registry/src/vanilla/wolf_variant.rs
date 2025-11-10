use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WolfVariant {}

impl Registrable for WolfVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:wolf_variant");
}
