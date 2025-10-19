use crate::REGISTRY_POPULATORS;
use crate::error::RegistryError;
use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::{RegistryBase, TypedRegistryBase};
use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::collections::HashMap;
use std::sync::Arc;

/// Central container for all dynamic registries (biomes, blocks, etc.).
///
/// To access its global instance, use `crab_registry::REGISTRY`.
#[derive(Default)]
pub struct RootRegistry {
    // FIXME: Implement string interning for registry IDs (#275)
    pub registries: HashMap<&'static str, Arc<dyn RegistryBase>, RandomState>,
}

impl RootRegistry {
    pub fn populated() -> Result<RootRegistry, RegistryError> {
        let mut registry = Self::default();

        for populate in REGISTRY_POPULATORS {
            populate(&mut registry)?;
        }

        Ok(registry)
    }

    pub fn register_registry<V: Registrable>(&mut self, key: RegistryKey<V>, registry: V::RegistryType) {
        self.registries.insert(key.key, Arc::new(registry));
    }

    pub fn all_registries(&self) -> impl ExactSizeIterator<Item = (&str, &dyn RegistryBase)> {
        self.registries.iter().map(|(&key, registry)| (key, registry.as_ref()))
    }

    pub fn registry_by_type<V: Registrable + 'static>(&self) -> &<V as Registrable>::RegistryType {
        // SAFETY: `RegistryKey` guarantees that the registry exists and has the correct type,
        //         so accessing it here is perfectly safe.
        let dynamic_registry = self
            .registries
            .get(V::REGISTRY.key)
            .ok_or(RegistryError::RegistryNotFound {
                registry: V::REGISTRY.key,
            })
            .unwrap();

        dynamic_registry
            .as_any()
            .downcast_ref::<<V as Registrable>::RegistryType>()
            .ok_or(RegistryError::RegistryTypeMismatch {
                registry: V::REGISTRY.key,
            })
            .unwrap()
    }

    pub fn key_to_id<T: Registrable>(&self, key: &NamespacedKey) -> Option<usize> {
        let registry = self.registry_by_type::<T>();
        registry.key_to_id(key)
    }

    pub fn try_key_to_id<T: Registrable>(&self, key: &NamespacedKey) -> Result<usize, RegistryError> {
        self.key_to_id::<T>(key)
            .ok_or_else(|| RegistryError::MissingKeyInRegistry {
                key: key.clone(),
                registry: T::REGISTRY.key,
            })
    }

    pub fn id_to_key<T: Registrable>(&self, key: usize) -> Option<&NamespacedKey> {
        let registry = self.registry_by_type::<T>();
        registry.id_to_key(key)
    }

    pub fn try_id_to_key<T: Registrable>(&self, id: usize) -> Result<&NamespacedKey, RegistryError> {
        let registry = self.registry_by_type::<T>();
        registry.id_to_key(id).ok_or(RegistryError::MissingIdInRegistry {
            id,
            registry: T::REGISTRY.key,
        })
    }

    pub fn value_by_key<T: Registrable>(&self, key: &NamespacedKey) -> Option<&Arc<T>> {
        let registry = self.registry_by_type::<T>();
        registry.value_by_key(key)
    }

    pub fn try_value_by_key<T: Registrable>(&self, key: &NamespacedKey) -> Result<&Arc<T>, RegistryError> {
        let registry = self.registry_by_type::<T>();
        registry
            .value_by_key(key)
            .ok_or_else(|| RegistryError::MissingKeyInRegistry {
                key: key.clone(),
                registry: T::REGISTRY.key,
            })
    }
}
