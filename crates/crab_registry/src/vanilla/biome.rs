use crate::registry_key::RegistryKey;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Biome {}

impl Biome {
    pub const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:worldgen/biome");

    pub const PLAINS: RegistryKey<Self> = RegistryKey::new("minecraft:plains");
    pub const THE_VOID: RegistryKey<Self> = RegistryKey::new("minecraft:the_void");
}
