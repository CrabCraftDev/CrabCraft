use crate::registry::Registry;
use crate::registry_key::RegistryKey;
use crate::vanilla::{Biome, BlockEntity, Empty};
use crate::{REGISTRY_POPULATORS, RootRegistry};
use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use linkme::distributed_slice;
use ordermap::OrderMap;
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

// FIXME: WRITE TESTS FOR ORDERED MAP
fn find_registry_files(root_dir: &Dir) -> HashMap<NamespacedKey, OrderMap<NamespacedKey, serde_json::Value>> {
    let mut result = HashMap::new();

    for dir in root_dir.dirs() {
        let mut names = OrderMap::new();
        for file in dir.files() {
            let path = Path::new(file.path());
            if let Some(stem) = path.file_stem() {
                names.insert(
                    NamespacedKey::minecraft(&stem.to_string_lossy()).unwrap(),
                    serde_json::from_slice(file.contents()).unwrap(),
                );
            }
        }

        for (key, value) in find_registry_files(dir) {
            result.insert(key, value);
        }

        let dir_namespaced_key = NamespacedKey::minecraft(&dir.path().to_string_lossy()).unwrap();
        result.insert(dir_namespaced_key, names);
    }

    result.retain(|_, v| !v.is_empty());
    result
}

#[distributed_slice(REGISTRY_POPULATORS)]
static BLOCK_ENTITIES_REGISTRY_POPULATOR: fn(&mut RootRegistry) = load_block_entities;

#[distributed_slice(REGISTRY_POPULATORS)]
static VANILLA_REGISTRY_POPULATOR: fn(&mut RootRegistry) = load_vanilla;

pub fn load_block_entities(registry: &mut RootRegistry) {
    const BLOCK_ENTITIES: &str = include_str!("../../../assets/block_entities.json");

    let block_entities: Vec<BlockEntity> = serde_json::from_str(BLOCK_ENTITIES).unwrap();

    let mut registry_instance = Registry::<BlockEntity>::new();

    for block_entity in block_entities.iter() {
        registry_instance.register(
            NamespacedKey::minecraft(&block_entity.name).unwrap(),
            (*block_entity).clone(),
        );
    }
    registry.register_registry(BlockEntity::REGISTRY, registry_instance);
}

pub fn init<T: Debug + Send + Sync + 'static + DeserializeOwned>(registry: &mut RootRegistry, id: NamespacedKey) {
    let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
    let mut data = find_registry_files(&root_dir);

    let entries = data.remove(&id).unwrap();

    let mut registry_instance = Registry::<T>::new();
    for (key, value) in entries {
        registry_instance.register(key, from_value(value).unwrap());
    }
    let leaked: &'static str = Box::leak(id.to_string().into_boxed_str());
    registry.register_registry(RegistryKey::new(leaked), registry_instance);
}

fn load_vanilla(registry: &mut RootRegistry) {
    init::<Empty>(registry, NamespacedKey::minecraft("cat_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("chicken_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("cow_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("damage_type").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("dimension_type").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("frog_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("painting_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("pig_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("wolf_sound_variant").unwrap());
    init::<Empty>(registry, NamespacedKey::minecraft("wolf_variant").unwrap());
    init::<Biome>(registry, NamespacedKey::from_str(Biome::REGISTRY.id).unwrap());
}

pub fn init_normal(registry: &mut RootRegistry) {
    let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
    let data = find_registry_files(&root_dir);

    for (registry_key, entries) in data {
        let mut registry_instance = Registry::<Biome>::new();
        for (key, value) in entries {
            registry_instance.register(key, from_value(value).unwrap());
        }
        let leaked: &'static str = Box::leak(registry_key.to_string().into_boxed_str());
        registry.register_registry(RegistryKey::new(leaked), registry_instance);
    }
}
