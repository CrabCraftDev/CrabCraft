use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

pub static REGISTRY: OnceLock<RegistryManager> = OnceLock::new();

/// RegistryManager is a struct that holds all dynamic registries like biomes.
/// Default data is obtained from the assets/datapack directory.
#[derive(Debug)]
pub struct RegistryManager {
    pub data: HashMap<NamespacedKey, Vec<NamespacedKey>>,
}

impl RegistryManager {
    pub fn new(data: HashMap<NamespacedKey, Vec<NamespacedKey>>) -> Self {
        Self { data }
    }

    pub fn get_numeric_id(&self, namespace: &NamespacedKey, entry: &NamespacedKey) -> Option<usize> {
        let entries = self.data.get(namespace)?;
        entries.iter().position(|x| x == entry)
    }

    pub fn get_length(&self, namespace: &NamespacedKey) -> Option<usize> {
        Some(self.data.get(namespace)?.len())
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
}

impl Default for RegistryManager {
    fn default() -> Self {
        let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
        let data = Self::find_registry_files(&root_dir);
        Self::new(data)
    }
}
