use std::collections::HashMap;

use crab_protocol::types::namespaced_key::NamespacedKey;
use crab_world::block_data::Snowy;
use crab_world::blocks::{init_blocks, GrassBlock};
use crab_world::{BlockType, BLOCKS};

#[test]
fn get_block_struct_information() {
    BLOCKS.get_or_init(init_blocks);

    let block = GrassBlock::default_state_id().unwrap();
    let properties = block.properties().unwrap();
    let id = block.namespaced_key().unwrap();

    let mut expected_properties = HashMap::new();
    // Note: 0 represents true and 1 represents false. By default, the grass block is not snowy.
    expected_properties.insert("snowy", 1);
    assert_eq!(expected_properties, properties);
    assert_eq!(id, &NamespacedKey::minecraft("grass_block").unwrap());
}

#[test]
fn create_block_from_properties() {
    BLOCKS.get_or_init(init_blocks);

    let mut hashmap = HashMap::new();
    hashmap.insert("snowy", "false".to_string());

    let block = GrassBlock::from_string_properties(&hashmap).unwrap();
    let is_snowy = block.property::<Snowy>().unwrap();

    assert!(!is_snowy);
}

#[test]
fn invalid_create_block_from_properties() {
    BLOCKS.get_or_init(init_blocks);

    let mut hashmap = HashMap::new();
    hashmap.insert("snowy", "cat".to_string());

    assert!(GrassBlock::from_string_properties(&hashmap).is_err());
}
