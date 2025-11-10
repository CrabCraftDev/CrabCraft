use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CatVariant {}

impl Registrable for CatVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:cat_variant");
}
