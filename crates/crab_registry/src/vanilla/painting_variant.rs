use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaintingVariant {}

impl Registrable for PaintingVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:painting_variant");
}
