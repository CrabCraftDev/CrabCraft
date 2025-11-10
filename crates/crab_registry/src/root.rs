use crate::REGISTRY_POPULATORS;
use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::{Registry, RegistryBase};
use crate::registry_id::RegistryId;
use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Central container for all dynamic registries (biomes, blocks, etc.).
///
/// To access its global instance, use `crab_registry::REGISTRY`.
#[derive(Default)]
pub struct RootRegistry {
    pub registries: HashMap<&'static str, Arc<dyn RegistryBase + Send + Sync>, RandomState>,
}

impl RootRegistry {
    pub fn populated() -> Self {
        let mut registry = Self::default();

        for populator in REGISTRY_POPULATORS {
            populator(&mut registry);
        }

        registry
    }

    pub fn register_registry<V: Debug + Send + Sync + 'static>(&mut self, key: RegistryKey<V>, registry: Registry<V>) {
        self.registries.insert(key.id, Arc::new(registry));
    }

    pub fn get_registry<V: Send + Sync + Registrable + 'static>(&self, key: &str) -> Option<&Registry<V>> {
        self.registries
            .get(key)
            .and_then(|arc| arc.as_ref().as_any().downcast_ref::<Registry<V>>())
    }

    pub fn registry_by_type<V: Send + Sync + Registrable + 'static>(&self) -> &Registry<V> {
        self.registries
            .get(&V::REGISTRY.id)
            .and_then(|arc| arc.as_ref().as_any().downcast_ref::<Registry<V>>())
            .unwrap()
    }

    pub fn get_all_registries(&self) -> HashMap<&str, &dyn RegistryBase> {
        self.registries
            .iter()
            .map(|(&k, v)| (k, v.as_ref() as &dyn RegistryBase))
            .collect()
    }

    pub fn key_to_id<T: Registrable + Send + Sync + 'static>(&self, key: RegistryKey<T>) -> usize {
        let registry = self.registry_by_type::<T>();

        registry
            .key_to_id(&key.as_namespaced_key())
            .expect("THIS SHOULD NEVER HAPPEN")
    }

    pub fn key_to_id2<T: Registrable + Send + Sync + 'static>(&self, key: &NamespacedKey) -> Option<usize> {
        let registry = self.registry_by_type::<T>();
        registry.key_to_id(key)
    }

    pub fn entry_by_key<T: Registrable>(&self, key: &impl RegistryId<T>) -> <impl RegistryId<T>>::AsValue {
        let registry = self.registry_by_type::<T>();
        key.entry_from_registry(registry)
    }
}

#[cfg(test)]
mod test {
    use crate::{RootRegistry, vanilla::Biome};

    #[test]
    fn test() {
        let r = RootRegistry::populated();
        r.entry_by_key::<Biome, _>(&0); // AAAAAA
        // Trzeba drugi paramietr podać
    }
}
