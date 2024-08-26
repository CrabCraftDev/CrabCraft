use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_world::blocks_registry::{BlockInformation, BlocksRegistry};
use crab_world::error::WorldError;
use crab_world::Block;
use std::collections::HashMap;

fn constructor(_hashmap: &HashMap<String, String>) -> Result<Block, WorldError> {
    Err(WorldError::BlocksNotInitialized)
}

fn get_properties(_id: u16) -> Result<HashMap<String, u16>, WorldError> {
    Err(WorldError::BlocksNotInitialized)
}

fn get_test_blocks_registry() -> BlocksRegistry {
    let mut blocks = BlocksRegistry::default();
    blocks
        .add_block(
            5,
            &NamespacedKey::minecraft("dirt").unwrap(),
            BlockInformation {
                from_properties: constructor,
                get_properties,
                starting_id: 0,
                default_id: 0,
            },
        )
        .unwrap();
    blocks
        .add_block(
            2,
            &NamespacedKey::minecraft("stone").unwrap(),
            BlockInformation {
                from_properties: constructor,
                get_properties,
                starting_id: 0,
                default_id: 0,
            },
        )
        .unwrap();
    blocks
}

#[test]
fn get_blocks_by_namespaced_key() {
    let blocks = get_test_blocks_registry();
    let dirt_block = blocks.get_by_id(&NamespacedKey::minecraft("dirt").unwrap()).unwrap();
    let stone_block = blocks.get_by_id(&NamespacedKey::minecraft("stone").unwrap()).unwrap();

    assert_eq!(dirt_block.starting_id, 0);
    assert_eq!(stone_block.starting_id, 5);
}

#[test]
fn get_invalid_blocks_by_namespaced_key() {
    let blocks = get_test_blocks_registry();
    assert!(blocks.get_by_id(&NamespacedKey::minecraft("test").unwrap()).is_err());
}

#[test]
fn get_blocks_by_id() {
    let blocks = get_test_blocks_registry();
    let dirt_block = blocks.get_block_from_id(0).unwrap();
    let another_dirt_block = blocks.get_block_from_id(4).unwrap();
    let stone_block = blocks.get_block_from_id(5).unwrap();
    let another_stone_block = blocks.get_block_from_id(6).unwrap();

    assert_eq!(dirt_block, &NamespacedKey::minecraft("dirt").unwrap());
    assert_eq!(another_dirt_block, &NamespacedKey::minecraft("dirt").unwrap());
    assert_eq!(stone_block, &NamespacedKey::minecraft("stone").unwrap());
    assert_eq!(another_stone_block, &NamespacedKey::minecraft("stone").unwrap());
}

#[test]
fn get_invalid_blocks_by_id() {
    let blocks = get_test_blocks_registry();
    assert!(blocks.get_block_from_id(213).is_err());
}
