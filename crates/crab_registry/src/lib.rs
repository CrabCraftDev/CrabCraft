use bimap::BiMap;
use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

pub static REGISTRY: OnceLock<RegistryManager> = OnceLock::new();

/// RegistryManager is a struct that holds all dynamic registries like biomes.
/// Default data is obtained from the assets/datapack directory.
#[derive(Debug)]
pub struct RegistryManager {
    pub data: HashMap<NamespacedKey, Vec<NamespacedKey>>,
    pub block_entities: BiMap<NamespacedKey, u16>,
}

impl RegistryManager {
    pub fn get_numeric_id(&self, registry: &NamespacedKey, entry: &NamespacedKey) -> Option<usize> {
        let entries = self.data.get(registry)?;
        entries.iter().position(|x| x == entry)
    }

    pub fn by_numeric_id(&self, registry: &NamespacedKey, id: usize) -> Option<&NamespacedKey> {
        let entries = self.data.get(registry)?;
        entries.get(id)
    }

    pub fn get_entry_count(&self, registry: &NamespacedKey) -> Option<usize> {
        Some(self.data.get(registry)?.len())
    }

    fn find_registry_files(root_dir: &Dir) -> HashMap<NamespacedKey, Vec<NamespacedKey>> {
        let mut result = HashMap::new();

        for dir in root_dir.dirs() {
            let mut names = Vec::new();
            for file in dir.files() {
                let path = Path::new(file.path());
                if let Some(stem) = path.file_stem() {
                    names.push(NamespacedKey::minecraft(&stem.to_string_lossy()).unwrap());
                }
            }

            for (key, value) in Self::find_registry_files(dir) {
                result.insert(key, value);
            }

            let dir_namespaced_key = NamespacedKey::minecraft(&dir.path().to_string_lossy()).unwrap();
            result.insert(dir_namespaced_key, names);
        }

        result.retain(|_, v| !v.is_empty());
        result
    }

    fn load_block_entities() -> BiMap<NamespacedKey, u16> {
        const BLOCK_ENTITIES: &str = include_str!("../../../assets/block_entities.json");

        let block_entities: Vec<BlockEntity> = serde_json::from_str(BLOCK_ENTITIES).unwrap();

        let mut bimap = BiMap::new();
        for (i, block_entity) in block_entities.iter().enumerate() {
            let key = NamespacedKey::minecraft(&block_entity.name).unwrap();
            bimap.insert(key.clone(), i as u16);
        }

        bimap
    }
}

impl Default for RegistryManager {
    fn default() -> Self {
        let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
        let data = Self::find_registry_files(&root_dir);
        let block_entities = Self::load_block_entities();

        Self { data, block_entities }
    }
}

#[derive(Debug, Deserialize)]
struct BlockEntity {
    pub name: String,
}
