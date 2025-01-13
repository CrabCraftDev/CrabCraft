use crate::world::worlds_storage::WORLDS_STORAGE;
use crate::world::World;
use crab_text::utils::namespaced_key::NamespacedKey;
use dashmap::mapref::one::{Ref, RefMut};
use derive_more::From;
use evenio::prelude::Component;

#[derive(Component, Clone, From, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct CurrentWorld(pub NamespacedKey);

impl CurrentWorld {
    pub fn get_world(&self) -> Option<Ref<NamespacedKey, World>> {
        WORLDS_STORAGE.worlds.get(&self.0)
    }

    pub fn get_world_mut(&self) -> Option<RefMut<NamespacedKey, World>> {
        WORLDS_STORAGE.worlds.get_mut(&self.0)
    }
}
