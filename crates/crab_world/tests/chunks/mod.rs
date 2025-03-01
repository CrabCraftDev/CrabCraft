use crab_registry::{REGISTRY, RegistryManager};
use crab_world::blocks::{BLOCKS, init_blocks};

mod bit_storage;
mod resize;
mod section;

fn init_registries() {
    REGISTRY.get_or_init(RegistryManager::default);
    BLOCKS.get_or_init(init_blocks);
}
