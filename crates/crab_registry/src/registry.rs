use crab_text::utils::namespaced_key::NamespacedKey;
pub use multi_id::*;
pub use root::*;
pub use simple::*;
use std::any::Any;
use std::sync::Arc;

mod multi_id;
mod root;
mod simple;

/// Object-safe facade for all concrete registries.
///
/// Lets the `RootRegistry` store heterogeneous `Registry<V>` instances in a
/// single map and perform basic operations like clearing and enumerating keys.
pub trait RegistryBase: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn keys(&self) -> Box<dyn Iterator<Item = &NamespacedKey> + '_>;

    fn key_to_id(&self, key: &NamespacedKey) -> Option<usize>;
    fn id_to_key(&self, id: usize) -> Option<&NamespacedKey>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait TypedRegistryBase: RegistryBase {
    type Value: Send + Sync + 'static;

    fn value_by_key(&self, key: &NamespacedKey) -> Option<&Arc<Self::Value>>;

    fn value_by_id(&self, id: usize) -> Option<&Arc<Self::Value>>;
}
