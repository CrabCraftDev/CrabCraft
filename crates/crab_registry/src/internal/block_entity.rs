use crate::REGISTRY_POPULATORS;
use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::Registry;
use crate::root::RootRegistry;
use crab_text::utils::namespaced_key::NamespacedKey;
use linkme::distributed_slice;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BlockEntity {
    pub name: String,
}

impl Registrable for BlockEntity {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("crabcraft:block_entity");
}

#[distributed_slice(REGISTRY_POPULATORS)]
pub fn load_block_entities(registry: &mut RootRegistry) {
    const BLOCK_ENTITIES: &str = include_str!("../../../../assets/block_entities.json");

    let block_entities: Vec<BlockEntity> = serde_json::from_str(BLOCK_ENTITIES).unwrap();

    let mut registry_instance = Registry::<BlockEntity>::default();

    for block_entity in block_entities.iter() {
        registry_instance.register(
            NamespacedKey::minecraft(&block_entity.name).unwrap(),
            (*block_entity).clone(),
        );
    }
    registry.register_registry(BlockEntity::REGISTRY, registry_instance);
}
