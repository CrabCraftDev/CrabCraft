use crate::utils::files::FILES;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::error::WorldError;
use crab_world::world::World;
use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

pub static WORLDS_STORAGE: LazyLock<WorldsStorage> = LazyLock::new(WorldsStorage::default);

#[derive(Debug, Default)]
pub struct WorldsStorage {
    pub worlds: Arc<DashMap<NamespacedKey, World>>,
}

impl WorldsStorage {
    pub fn save(&self) -> Result<(), WorldError> {
        for world in self.worlds.iter() {
            world.save()?;
        }

        Ok(())
    }

    pub fn init_dimension(&self, dimension: NamespacedKey, min_y: i16) {
        self.worlds.insert(
            dimension.clone(),
            World {
                min_y,
                identifier: dimension.clone(),
                path: FILES.world_region_path(&dimension).unwrap(),
                chunks: Default::default(),
                regions: Default::default(),
            },
        );
    }
}
