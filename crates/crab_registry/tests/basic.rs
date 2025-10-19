use crab_registry::registry_key::{Registry, RegistryKey};
use crab_registry::{REGISTRY, RootRegistry};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
struct Biome {
    pub name: &'static str,
}

impl Biome {
    pub const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:biomes");

    pub const PLAINS: RegistryKey<Self> = RegistryKey::new("minecraft:plains");
    pub const THE_VOID: RegistryKey<Self> = RegistryKey::new("minecraft:the_void");
}

#[test]
fn main() {
    let mut biomes = Registry::<Biome>::new();
    biomes.register(
        NamespacedKey::from_str(Biome::PLAINS.id).unwrap(),
        Biome { name: "Plains" },
    );
    biomes.register(
        NamespacedKey::from_str(Biome::THE_VOID.id).unwrap(),
        Biome { name: "The Void" },
    );

    let mut registry = RootRegistry::new();
    registry.register_registry(Biome::REGISTRY, biomes);
    REGISTRY.store(Arc::new(registry));

    let root_registry = REGISTRY.load();
    let registry = root_registry.get_registry(Biome::REGISTRY).unwrap();

    let plains = registry
        .get_by_key(&NamespacedKey::from_str(Biome::PLAINS.id).unwrap())
        .unwrap();
    assert_eq!(plains.name, "Plains");

    let plains_id = registry
        .key_to_id(&NamespacedKey::from_str(Biome::PLAINS.id).unwrap())
        .unwrap();
    let plains2 = registry.get_by_id(plains_id).unwrap();
    assert_eq!(plains2.name, "Plains");
}
