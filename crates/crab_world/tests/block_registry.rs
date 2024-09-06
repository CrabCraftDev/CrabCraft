use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_world::block_registry::{BlockInformation, BlockRegistry};
use crab_world::error::WorldError;
use crab_world::Block;
use std::collections::HashMap;

fn from_properties(_hashmap: &HashMap<&str, u16>) -> Result<Block, WorldError> {
    unreachable!()
}

fn from_string_properties(_hashmap: &HashMap<&str, String>) -> Result<Block, WorldError> {
    unreachable!()
}

fn properties(_id: u16) -> Result<HashMap<&'static str, u16>, WorldError> {
    unreachable!()
}

fn get_example_block_registry() -> BlockRegistry {
    let mut blocks = BlockRegistry::default();
    blocks
        .add_block(
            5,
            &NamespacedKey::minecraft("dirt").unwrap(),
            BlockInformation {
                from_properties,
                from_string_properties,
                properties,
                starting_state_id: 0,
                default_state_id: 0,
            },
        )
        .unwrap();
    blocks
        .add_block(
            2,
            &NamespacedKey::minecraft("stone").unwrap(),
            BlockInformation {
                from_properties,
                from_string_properties,
                properties,
                starting_state_id: 0,
                default_state_id: 0,
            },
        )
        .unwrap();
    blocks
}

#[test]
fn get_blocks_by_namespaced_key() {
    let blocks = get_example_block_registry();
    let dirt_block = blocks
        .block_information_by_namespaced_key(&NamespacedKey::minecraft("dirt").unwrap())
        .unwrap();
    let stone_block = blocks
        .block_information_by_namespaced_key(&NamespacedKey::minecraft("stone").unwrap())
        .unwrap();

    assert_eq!(dirt_block.starting_state_id, 0);
    assert_eq!(stone_block.starting_state_id, 5);
}

#[test]
fn get_invalid_blocks_by_namespaced_key() {
    let blocks = get_example_block_registry();
    assert!(blocks
        .block_information_by_namespaced_key(&NamespacedKey::minecraft("test").unwrap())
        .is_err());
}

#[test]
fn get_blocks_by_id() {
    let blocks = get_example_block_registry();
    let dirt_block = blocks.namespaced_key_by_state_id(0).unwrap();
    let another_dirt_block = blocks.namespaced_key_by_state_id(4).unwrap();
    let stone_block = blocks.namespaced_key_by_state_id(5).unwrap();
    let another_stone_block = blocks.namespaced_key_by_state_id(6).unwrap();

    assert_eq!(dirt_block, &NamespacedKey::minecraft("dirt").unwrap());
    assert_eq!(another_dirt_block, &NamespacedKey::minecraft("dirt").unwrap());
    assert_eq!(stone_block, &NamespacedKey::minecraft("stone").unwrap());
    assert_eq!(another_stone_block, &NamespacedKey::minecraft("stone").unwrap());
}

#[test]
fn get_invalid_blocks_by_id() {
    let blocks = get_example_block_registry();
    assert!(blocks.namespaced_key_by_state_id(213).is_err());
}
