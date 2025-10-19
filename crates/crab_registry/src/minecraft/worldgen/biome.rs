use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::SimpleRegistry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Biome {}

impl Registrable for Biome {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:worldgen/biome");
}

impl Biome {
    pub const PLAINS: RegistryKey<Self> = RegistryKey::new("minecraft:plains");
    pub const THE_VOID: RegistryKey<Self> = RegistryKey::new("minecraft:the_void");
}
