use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Biome {}

impl Registrable for Biome {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:worldgen/biome");
}

impl Biome {
    pub const PLAINS: RegistryKey<Self> = RegistryKey::new("minecraft:plains");
    pub const THE_VOID: RegistryKey<Self> = RegistryKey::new("minecraft:the_void");
}
