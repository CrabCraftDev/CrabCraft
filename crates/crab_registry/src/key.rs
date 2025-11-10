use crab_text::utils::namespaced_key::NamespacedKey;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

use crate::Registrable;
use crate::registry::Registry;
use crate::registry_id::RegistryId;
// use crate::registry_id::RegistryId;

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

impl<T: Registrable> RegistryId<T> for RegistryKey<T> {
    type AsValue = Arc<T>;
    type AsNumericId = usize;
    type AsNamespacedKey = NamespacedKey;

    fn entry_from_registry(&self, registry: &Registry<T>) -> Self::AsValue {
        registry
            .get_by_key(&self.as_namespaced_key())
            .expect("RegistryKey guarantee of value existing in a registry was violated")
    }

    fn numeric_id_from_registry(&self, registry: &Registry<T>) -> Self::AsNumericId {
        registry
            .key_to_id(&self.as_namespaced_key())
            .expect("RegistryKey guarantee of value existing in a registry was violated")
    }

    fn namespaced_key_from_registry<'ns>(&self, _registry: &Registry<T>) -> Self::AsNamespacedKey {
        self.as_namespaced_key()
    }
}
