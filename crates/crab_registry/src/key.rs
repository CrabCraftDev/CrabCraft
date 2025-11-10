use crab_text::utils::namespaced_key::NamespacedKey;
use std::marker::PhantomData;
use std::str::FromStr;

/// Type-safe identifier for a registry entry.
///
/// Intended for situations where you are certain that both the registry and the key
/// are already registered. Use with caution in const contexts.
#[derive(Clone)]
pub struct RegistryKey<T> {
    // FIXME: Prefer `NamespacedKey` for type safety, but it cannot be used in a `const` (#275)
    pub id: &'static str,
    _marker: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn as_namespaced_key(&self) -> NamespacedKey {
        NamespacedKey::from_str(self.id).unwrap()
    }
}
