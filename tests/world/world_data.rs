use crab_craft::world::World;
use crab_registry::{REGISTRY, RegistryManager};
use crab_world::blocks::{BLOCKS, init_blocks};

#[test]
fn load_missing_region_file() {
    BLOCKS.get_or_init(init_blocks);
    REGISTRY.get_or_init(RegistryManager::default);

    let world = World::default();
    assert!(world.get_chunk(i32::MAX, 0).is_none());

    let result = world.get_chunk_or_load(i32::MAX, 0);
    assert!(result.is_ok());
}
