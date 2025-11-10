use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use ordermap::OrderMap;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::vec::IntoIter;

/// Object-safe facade for all concrete registries.
///
/// Lets the `RootRegistry` store heterogeneous `Registry<V>` instances in a
/// single map and perform basic operations like clearing and enumerating keys.
pub trait RegistryBase: Any + Send + Sync {
    fn clear(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn ids(&self) -> IntoIter<&NamespacedKey>;
}

/// Concrete registry for values of type `V` keyed by `NamespacedKey`.
#[derive(Debug)]
pub struct Registry<V> {
    // TODO: Introduce different registries, in some of them we may even not need hashmap
    key_to_id: OrderMap<NamespacedKey, Arc<V>, RandomState>,

    // TODO: Either we can keep this and add nohash, or create multiple registry kinds
    alias_to_main: HashMap<usize, usize, RandomState>,
}

impl<V> Default for Registry<V> {
    fn default() -> Self {
        Self {
            key_to_id: OrderMap::default(),
            alias_to_main: HashMap::default(),
        }
    }
}

impl<V> Registry<V> {
    pub fn register(&mut self, key: NamespacedKey, value: V) -> usize {
        let id = self.key_to_id.len();
        self.key_to_id.insert(key.clone(), Arc::new(value));
        id
    }

    pub fn add_alias(&mut self, alias_id: usize, target_id: usize) {
        self.alias_to_main.insert(alias_id, target_id);
    }

    pub fn get_by_id(&self, id: usize) -> Option<Arc<V>> {
        let real = self.alias_to_main.get(&id).copied().unwrap_or(id);
        self.key_to_id.get_index(real).map(|e| e.1.clone())
    }
    pub fn get_by_key(&self, key: &NamespacedKey) -> Option<Arc<V>> {
        self.key_to_id.get(key).cloned()
    }
    pub fn id_to_key(&self, id: usize) -> Option<&NamespacedKey> {
        let real = self.alias_to_main.get(&id).copied().unwrap_or(id);
        self.key_to_id.get_index(real).map(|e| e.0)
    }
    pub fn key_to_id(&self, key: &NamespacedKey) -> Option<usize> {
        self.key_to_id.get_index_of(key)
    }

    pub fn len(&self) -> usize {
        self.key_to_id.len() + self.alias_to_main.len()
    }

    pub fn is_empty(&self) -> bool {
        self.key_to_id.is_empty()
    }
}

impl<V> IntoIterator for Registry<V> {
    type Item = Arc<V>;
    type IntoIter = IntoIter<Arc<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.key_to_id
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<V: Send + Sync + Any> RegistryBase for Registry<V> {
    fn clear(&mut self) {
        self.key_to_id.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn ids(&self) -> IntoIter<&NamespacedKey> {
        self.key_to_id.keys().collect::<Vec<_>>().into_iter()
    }
}
