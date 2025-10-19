use std::collections::HashMap;
use strum::{Display, EnumString};

use crab_macros::BlockData;
use crab_world::blocks::BlockType;
use crab_world::blocks::GrassBlock;
use crab_world::blocks::block_data::{BlockData, Snowy};

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
struct ComplexBlockData {
    pub boolean: bool,
    pub number: u16,
    pub enum_value: ExampleEnum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[repr(u16)]
enum ExampleEnum {
    A,
    B,
    C,
}

#[test]
fn complex_block_data() {
    let properties = HashMap::from([
        ("boolean", "true".to_string()),
        ("number", "3".to_string()),
        ("enum_value", "b".to_string()),
    ]);

    let data: ComplexBlockData = ComplexBlockData::from_properties(properties).unwrap();
    assert!(data.boolean);
    assert_eq!(data.number, 3);
    assert_eq!(data.enum_value, ExampleEnum::B);
}

#[test]
fn invalid_block_data() {
    let properties = HashMap::from([("boolean", "xd".to_string())]);

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn missing_block_data() {
    let properties = HashMap::new();

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn get_block_struct_information() {
    let mut block = GrassBlock::default_state_id().unwrap();

    assert!(!block.property::<Snowy>().unwrap());
    block.apply_property::<Snowy>(&true).unwrap();
    assert!(block.property::<Snowy>().unwrap());
}
