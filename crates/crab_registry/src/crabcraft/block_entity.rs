use crate::REGISTRY_POPULATORS;
use crate::error::RegistryError;
use crate::key::RegistryKey;
use crate::registrable::Registrable;
use crate::registry::RootRegistry;
use crate::registry::SimpleRegistry;
use crab_text::utils::namespaced_key::NamespacedKey;
use linkme::distributed_slice;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BlockEntity {
    pub name: String,
}

impl Registrable for BlockEntity {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("crabcraft:block_entity");
}

#[distributed_slice(REGISTRY_POPULATORS)]
pub fn load_block_entities(registry: &mut RootRegistry) -> Result<(), RegistryError> {
    const BLOCK_ENTITIES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/block_entities.json"));

    let block_entities: Vec<BlockEntity> =
        serde_json::from_str(BLOCK_ENTITIES).map_err(|source| RegistryError::JsonParseError {
            source,
            path: "assets/block_entities.json".to_string(),
        })?;

    let mut registry_instance = SimpleRegistry::<BlockEntity>::default();

    for block_entity in block_entities.into_iter() {
        registry_instance.register(NamespacedKey::minecraft(&block_entity.name)?, block_entity);
    }
    registry.register_registry(BlockEntity::REGISTRY, registry_instance);

    Ok(())
}
