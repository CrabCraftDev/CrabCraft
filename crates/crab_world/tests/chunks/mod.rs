use crab_registry::{RegistryManager, REGISTRY};
use crab_world::blocks::{init_blocks, BLOCKS};

mod bit_storage;
mod resize;
mod section;

fn init_registries() {
    REGISTRY.get_or_init(RegistryManager::default);
    BLOCKS.get_or_init(init_blocks);
}
