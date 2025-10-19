use crate::Registrable;
use crate::registry::{RegistryBase, TypedRegistryBase};
use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use ordermap::OrderMap;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// A simple registry mapping `NamespacedKey`s or numeric ids to values of type `V`.
#[derive(Debug)]
pub struct SimpleRegistry<V: Registrable> {
    key_to_value: OrderMap<NamespacedKey, Arc<V>, RandomState>,
}

impl<V: Registrable> Default for SimpleRegistry<V> {
    fn default() -> Self {
        Self {
            key_to_value: OrderMap::default(),
        }
    }
}

impl<V: Registrable> SimpleRegistry<V> {
    pub fn register(&mut self, key: NamespacedKey, value: V) -> usize {
        let id = self.key_to_value.len();
        self.key_to_value.insert(key.clone(), Arc::new(value));
        id
    }
}

impl<V: Registrable> TypedRegistryBase for SimpleRegistry<V> {
    type Value = V;

    fn value_by_key(&self, key: &NamespacedKey) -> Option<&Arc<Self::Value>> {
        self.key_to_value.get(key)
    }

    fn value_by_id(&self, id: usize) -> Option<&Arc<Self::Value>> {
        self.key_to_value.get_index(id).map(|(_, value)| value)
    }
}

impl<V: Registrable> RegistryBase for SimpleRegistry<V> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &NamespacedKey> + '_> {
        Box::new(self.key_to_value.keys())
    }

    fn key_to_id(&self, key: &NamespacedKey) -> Option<usize> {
        self.key_to_value.get_index_of(key)
    }

    fn id_to_key(&self, id: usize) -> Option<&NamespacedKey> {
        self.key_to_value.get_index(id).map(|e| e.0)
    }

    fn len(&self) -> usize {
        self.key_to_value.len()
    }

    fn is_empty(&self) -> bool {
        self.key_to_value.is_empty()
    }
}
