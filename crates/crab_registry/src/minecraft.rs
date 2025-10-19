use crate::REGISTRY_POPULATORS;
use crate::error::RegistryError;
use crate::registrable::Registrable;
use crate::registry::RootRegistry;
use crate::registry::SimpleRegistry;
use crab_text::utils::namespaced_key::NamespacedKey;
use include_dir::{Dir, include_dir};
use linkme::distributed_slice;
use ordermap::OrderMap;
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

mod cat_variant;
mod chicken_variant;
mod cow_variant;
mod damage_type;
mod dimension_type;
mod frog_variant;
mod painting_variant;
mod pig_variant;
mod wolf_sound_variant;
mod wolf_variant;
mod worldgen;

pub use cat_variant::*;
pub use chicken_variant::*;
pub use cow_variant::*;
pub use damage_type::*;
pub use dimension_type::*;
pub use frog_variant::*;
pub use painting_variant::*;
pub use pig_variant::*;
pub use wolf_sound_variant::*;
pub use wolf_variant::*;
pub use worldgen::*;

#[distributed_slice(REGISTRY_POPULATORS)]
fn populate_vanilla(registry: &mut RootRegistry) -> Result<(), RegistryError> {
    let root_dir = include_dir!("$CARGO_MANIFEST_DIR/../../assets/datapack");
    let mut data = find_registry_files(&root_dir)?;

    populate_single::<CatVariant>(registry, &mut data)?;
    populate_single::<ChickenVariant>(registry, &mut data)?;
    populate_single::<CowVariant>(registry, &mut data)?;
    populate_single::<DamageType>(registry, &mut data)?;
    populate_single::<DimensionType>(registry, &mut data)?;
    populate_single::<FrogVariant>(registry, &mut data)?;
    populate_single::<PaintingVariant>(registry, &mut data)?;
    populate_single::<PigVariant>(registry, &mut data)?;
    populate_single::<WolfVariant>(registry, &mut data)?;
    populate_single::<WolfSoundVariant>(registry, &mut data)?;
    populate_single::<Biome>(registry, &mut data)?;

    Ok(())
}

fn populate_single<T>(
    registry: &mut RootRegistry,
    data: &mut HashMap<NamespacedKey, OrderMap<NamespacedKey, serde_json::Value>>,
) -> Result<(), RegistryError>
where
    T: Registrable<RegistryType = SimpleRegistry<T>> + Debug + Send + Sync + 'static + DeserializeOwned,
{
    let key = T::REGISTRY.as_namespaced_key();
    let entries = data.remove(&key).ok_or(RegistryError::MissingVanillaRegistry {
        registry: T::REGISTRY.key,
    })?;

    let mut registry_instance = SimpleRegistry::<T>::default();
    for (key, value) in entries {
        let value = from_value(value).map_err(|source| RegistryError::JsonParseError {
            path: key.to_string(),
            source,
        })?;

        registry_instance.register(key, value);
    }

    registry.register_registry(T::REGISTRY, registry_instance);

    Ok(())
}

fn find_registry_files(
    root_dir: &Dir,
) -> Result<HashMap<NamespacedKey, OrderMap<NamespacedKey, serde_json::Value>>, RegistryError> {
    let mut result = HashMap::new();

    for directory in root_dir.dirs() {
        let mut names = OrderMap::new();
        for file in directory.files() {
            let path = Path::new(file.path());
            if let Some(stem) = path.file_stem() {
                let namespace = NamespacedKey::minecraft(&stem.to_string_lossy())?;
                let value =
                    serde_json::from_slice(file.contents()).map_err(|source| RegistryError::JsonParseError {
                        path: path.to_string_lossy().to_string(),
                        source,
                    })?;
                names.insert(namespace, value);
            }
        }

        for (key, value) in find_registry_files(directory)? {
            result.insert(key, value);
        }

        let directory_name = directory.path().to_string_lossy();
        result.insert(NamespacedKey::minecraft(&directory_name)?, names);
    }

    result.retain(|_, v| !v.is_empty());
    Ok(result)
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
        // to make sure the order is *always* preserved by our implementation
        for _ in 0..u8::MAX {
            let data = find_registry_files(&dir).unwrap();

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
