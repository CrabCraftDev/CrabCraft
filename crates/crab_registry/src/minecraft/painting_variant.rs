use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::SimpleRegistry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaintingVariant {}

impl Registrable for PaintingVariant {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:painting_variant");
}
