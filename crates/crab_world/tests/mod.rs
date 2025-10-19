mod blocks;
mod chunks;
mod world;

pub fn init_registries() {
    use crab_registry::minecraft::Biome;
    use crab_registry::{REGISTRY, RootRegistry, init, load_block_entities};
    use crab_text::utils::namespaced_key::NamespacedKey;
    use crab_world::blocks::block_registry::BlockInformation;
    use crab_world::blocks::init_blocks;
    use std::str::FromStr;
    use std::sync::Arc;

    let mut registry = RootRegistry::new();
    registry.register_registry(BlockInformation::REGISTRY, init_blocks());
    init::<Biome>(&mut registry, NamespacedKey::from_str(Biome::REGISTRY.id).unwrap());
    load_block_entities(&mut registry);
    REGISTRY.store(Arc::new(registry));
}
