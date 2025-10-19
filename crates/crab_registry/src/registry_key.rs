use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use ordermap::OrderMap;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistryKey<T> {
    // FIXME: Use NamespacedKey, but we cannot do it for now, because it is not const
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
}

pub trait RegistryBase: Any + Send + Sync {
    fn clear(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn ids(&self) -> &[NamespacedKey];
}

pub struct Registry<V> {
    values: Vec<Arc<V>>,
    // FIXME: Introduce different registries, in some of them we may even not need hashmap
    key_to_id: OrderMap<NamespacedKey, usize, RandomState>,
    id_to_key: Vec<NamespacedKey>,
}

impl<V> Default for Registry<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Registry<V> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            key_to_id: OrderMap::default(),
            id_to_key: Vec::new(),
        }
    }

    pub fn register(&mut self, key: NamespacedKey, value: V) -> usize {
        let id = self.values.len();
        self.values.push(Arc::new(value));
        self.key_to_id.insert(key.clone(), id);
        self.id_to_key.push(key);
        id
    }

    pub fn get_by_id(&self, id: usize) -> Option<Arc<V>> {
        self.values.get(id).cloned()
    }
    pub fn get_by_key(&self, key: &NamespacedKey) -> Option<Arc<V>> {
        self.key_to_id.get(key).and_then(|&id| self.get_by_id(id))
    }
    pub fn id_to_key(&self, id: usize) -> Option<&NamespacedKey> {
        self.id_to_key.get(id)
    }
    pub fn key_to_id(&self, key: &NamespacedKey) -> Option<usize> {
        self.key_to_id.get(key).cloned()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<V> IntoIterator for Registry<V> {
    type Item = Arc<V>;
    type IntoIter = std::vec::IntoIter<Arc<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<V: Send + Sync + Any> RegistryBase for Registry<V> {
    fn clear(&mut self) {
        self.key_to_id.clear();
        self.id_to_key.clear();
        self.values.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn ids(&self) -> &[NamespacedKey] {
        &self.id_to_key
    }
}

/// RegistryManager is a struct that holds all dynamic registries like biomes.
/// Default data is obtained from the assets/datapack directory.
pub struct RootRegistry {
    pub registries: HashMap<&'static str, Arc<dyn RegistryBase + Send + Sync>, RandomState>,
}

impl Default for RootRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl RootRegistry {
    pub fn new() -> Self {
        Self {
            registries: HashMap::default(),
        }
    }

    pub fn register_registry<V: Debug + Send + Sync + 'static>(&mut self, key: RegistryKey<V>, registry: Registry<V>) {
        self.registries.insert(key.id, Arc::new(registry));
    }

    pub fn get_registry<V: Send + Sync + 'static>(&self, key: RegistryKey<V>) -> Option<&Registry<V>> {
        self.registries
            .get(&key.id)
            .and_then(|arc| arc.as_ref().as_any().downcast_ref::<Registry<V>>())
    }

    pub fn get_all_registries(&self) -> HashMap<&str, &dyn RegistryBase> {
        self.registries
            .iter()
            .map(|(&k, v)| (k, v.as_ref() as &dyn RegistryBase))
            .collect()
    }
}
