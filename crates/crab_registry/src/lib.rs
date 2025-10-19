pub use crate::registry_key::RootRegistry;
use crate::registry_key::{Registry, RegistryKey};
use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::sync::LazyLock;

use crate::minecraft::{Biome, BlockEntity};
use arc_swap::ArcSwap;
use ordermap::OrderMap;
use serde::de::DeserializeOwned;
use serde_json::from_value;

pub mod error;
pub mod minecraft;
pub mod registry_key;

pub static REGISTRY: LazyLock<ArcSwap<RootRegistry>> = LazyLock::new(|| ArcSwap::from_pointee(RootRegistry::new()));

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
