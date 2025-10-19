use crate::init_registries;
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::blocks::BlockType;
use crab_world::blocks::GrassBlock;
use crab_world::blocks::block_data::Snowy;
use std::collections::HashMap;

#[test]
fn get_block_struct_information() {
    init_registries();

    let block = GrassBlock::default_state_id().unwrap();
    let (id, properties) = block.properties().unwrap();

    let mut expected_properties = HashMap::new();
    expected_properties.insert("snowy", "false".to_string());
    assert_eq!(expected_properties, properties);
    assert_eq!(id, NamespacedKey::minecraft("grass_block").unwrap());
}

#[test]
fn create_block_from_properties() {
    init_registries();

    let mut hashmap = HashMap::new();
    hashmap.insert("snowy", "false".to_string());

    let block = GrassBlock::from_properties(&hashmap).unwrap();
    let is_snowy = block.property::<Snowy>().unwrap();

    assert!(!is_snowy);
}

#[test]
fn invalid_create_block_from_properties() {
    init_registries();

    let mut hashmap = HashMap::new();
    hashmap.insert("snowy", "cat".to_string());

    assert!(GrassBlock::from_properties(&hashmap).is_err());
}
