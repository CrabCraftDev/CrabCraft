use crate::{REGISTRY, Registrable};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::marker::PhantomData;
use std::str::FromStr;

/// Type-safe identifier for a registry entry.
///
/// Intended for situations where you are certain that both the registry and the key
/// are already registered. Use it only in const context, where are you sure value exists.
#[derive(Clone)]
pub struct RegistryKey<T: Registrable> {
    // FIXME: Prefer `NamespacedKey` for type safety, but it cannot be used in a `const` (#275)
    pub key: &'static str,
    _marker: PhantomData<T>,
}

impl<T: Registrable + 'static> RegistryKey<T> {
    pub const fn new(id: &'static str) -> Self {
        Self {
            key: id,
            _marker: PhantomData,
        }
    }

    pub fn to_numeric_id(&self) -> usize {
        let root = REGISTRY.load();
        let key = self.as_namespaced_key();
        match root.as_ref().try_key_to_id::<T>(&key) {
            Ok(id) => id,
            Err(error) => unreachable!("RegistryKey couldn't be converted to ID: {}", error),
        }
    }

    pub fn as_namespaced_key(&self) -> NamespacedKey {
        match NamespacedKey::from_str(self.key) {
            Ok(value) => value,
            Err(error) => unreachable!("RegistryKey has invalid namespaced key '{}': {}", self.key, error),
        }
    }
}
