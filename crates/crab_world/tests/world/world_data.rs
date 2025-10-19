use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::world::World;
use std::sync::Arc;

#[test]
fn load_missing_region_file() {
    let world = World {
        identifier: NamespacedKey::minecraft("world").unwrap(),
        path: Default::default(),
        min_y: -64,
        chunks: Arc::new(Default::default()),
        regions: Default::default(),
    };
    assert!(world.get_chunk(i32::MAX, 0).is_none());

    let result = world.get_chunk_or_load(i32::MAX, 0);
    assert!(result.is_ok());
}
