use crate::Registrable;
use crate::registry::{RegistryBase, TypedRegistryBase};
use ahash::RandomState;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// A registry that allows mapping multiple numeric identifiers to the same key
/// or value.
///
/// Intended primarily as an internal mechanism used by the current block
/// implementation. It has no broader use cases outside this context.
#[derive(Debug)]
pub struct MultiIdRegistry<V: Registrable> {
    key_to_value: HashMap<Arc<NamespacedKey>, Arc<V>, RandomState>,
    id_to_key: Vec<Arc<NamespacedKey>>,
}

impl<V: Registrable> Default for MultiIdRegistry<V> {
    fn default() -> Self {
        Self {
            key_to_value: HashMap::default(),
            id_to_key: Vec::default(),
        }
    }
}

impl<V: Registrable> MultiIdRegistry<V> {
    pub fn register_multiple(&mut self, key: NamespacedKey, value: V, size: usize) {
        let key_arc = Arc::new(key);
        self.key_to_value
            .entry(key_arc.clone())
            .or_insert_with(|| Arc::new(value));

        for _ in 0..size {
            self.id_to_key.push(key_arc.clone());
        }
    }
}

impl<V: Registrable> TypedRegistryBase for MultiIdRegistry<V> {
    type Value = V;

    fn value_by_key(&self, key: &NamespacedKey) -> Option<&Arc<Self::Value>> {
        self.key_to_value.get(key)
    }

    fn value_by_id(&self, id: usize) -> Option<&Arc<Self::Value>> {
        self.value_by_key(self.id_to_key.get(id)?)
    }
}

impl<V: Registrable> RegistryBase for MultiIdRegistry<V> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &NamespacedKey> + '_> {
        Box::new(self.id_to_key.iter().map(|key| key.as_ref()))
    }

    fn key_to_id(&self, key: &NamespacedKey) -> Option<usize> {
        self.id_to_key.iter().position(|k| k.as_ref() == key)
    }

    fn id_to_key(&self, id: usize) -> Option<&NamespacedKey> {
        self.id_to_key.get(id).map(|k| k.as_ref())
    }

    fn len(&self) -> usize {
        self.id_to_key.len()
    }

    fn is_empty(&self) -> bool {
        self.id_to_key.is_empty()
    }
}
