use crate::REGISTRY_POPULATORS;
use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::Registry;
use crate::root::RootRegistry;
use crate::vanilla::*;
use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use linkme::distributed_slice;
use ordermap::OrderMap;
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

#[distributed_slice(REGISTRY_POPULATORS)]
fn populate_vanilla(registry: &mut RootRegistry) {
    let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
    let data = find_registry_files(&root_dir);

    populate_single::<CatVariant>(registry, &data);
    populate_single::<ChickenVariant>(registry, &data);
    populate_single::<CowVariant>(registry, &data);
    populate_single::<DamageType>(registry, &data);
    populate_single::<DimensionType>(registry, &data);
    populate_single::<FrogVariant>(registry, &data);
    populate_single::<PaintingVariant>(registry, &data);
    populate_single::<PigVariant>(registry, &data);
    populate_single::<WolfVariant>(registry, &data);
    populate_single::<WolfSoundVariant>(registry, &data);
    populate_single::<Biome>(registry, &data);
}

fn populate_single<T>(
    registry: &mut RootRegistry,
    data: &HashMap<NamespacedKey, OrderMap<NamespacedKey, serde_json::Value>>,
) where
    T: Registrable + Debug + Send + Sync + 'static + DeserializeOwned,
{
    let id = T::REGISTRY.as_namespaced_key();
    let entries = data.get(&id).expect("registry id missing in datapack");

    let mut registry_instance = Registry::<T>::default();
    for (key, value) in entries {
        registry_instance.register(key.clone(), from_value(value.clone()).unwrap());
    }

    let leaked: &'static str = Box::leak(id.to_string().into_boxed_str());
    registry.register_registry(RegistryKey::new(leaked), registry_instance);
}

fn find_registry_files(root_dir: &Dir) -> HashMap<NamespacedKey, OrderMap<NamespacedKey, serde_json::Value>> {
    let mut result = HashMap::new();

    for directory in root_dir.dirs() {
        let mut names = OrderMap::new();
        for file in directory.files() {
            let path = Path::new(file.path());
            if let Some(stem) = path.file_stem() {
                names.insert(
                    NamespacedKey::minecraft(&stem.to_string_lossy()).unwrap(),
                    serde_json::from_slice(file.contents()).unwrap(),
                );
            }
        }

        for (key, value) in find_registry_files(directory) {
            result.insert(key, value);
        }

        let dir_namespaced_key = NamespacedKey::minecraft(&directory.path().to_string_lossy()).unwrap();
        result.insert(dir_namespaced_key, names);
    }

    result.retain(|_, v| !v.is_empty());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::{Dir, DirEntry, File};

    #[test]
    fn test_find_registry_files_ordering() {
        let animals: &[DirEntry] = &[
            DirEntry::File(File::new("cat.json", br#"{"v": 1}"#)),
            DirEntry::File(File::new("dog.json", br#"{"v": 2}"#)),
        ];

        let entries: &[DirEntry] = &[DirEntry::Dir(Dir::new("animals", animals))];

        let dir = Dir::new("registry_test_dir", entries);

        // The order in Hashmaps may be random, so we run it multiple times
        // to make sure the order is *always* preserved by thez OrderMap
        for _ in 0..u8::MAX {
            let data = find_registry_files(&dir);

            let animals_key = NamespacedKey::minecraft("animals").unwrap();
            assert!(data.contains_key(&animals_key));

            let animals_map = &data[&animals_key];
            let keys: Vec<_> = animals_map.keys().map(|k| k.to_string()).collect();
            assert_eq!(keys, vec!["minecraft:cat", "minecraft:dog"]);

            assert_eq!(
                animals_map[&NamespacedKey::minecraft("cat").unwrap()],
                serde_json::json!({"v":1})
            );
            assert_eq!(
                animals_map[&NamespacedKey::minecraft("dog").unwrap()],
                serde_json::json!({"v":2})
            );
        }
    }
}
