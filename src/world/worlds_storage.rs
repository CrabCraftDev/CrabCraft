use crate::world::world_data::World;
use crab_protocol::types::namespaced_key::NamespacedKey;
use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

pub static WORLDS_STORAGE: LazyLock<WorldsStorage> = LazyLock::new(WorldsStorage::default);

#[derive(Debug, Default)]
pub struct WorldsStorage {
    pub worlds: Arc<DashMap<NamespacedKey, World>>,
}

impl WorldsStorage {
    pub fn init_dimension(&self, dimension: NamespacedKey, min_y: i16) {
        self.worlds.insert(
            dimension,
            World {
                min_y,
                ..Default::default()
            },
        );
    }
}
