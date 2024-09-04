use std::collections::HashMap;

use strum::FromRepr;

use crab_macros::BlockData;
use crab_world::block_data::{BlockData, Snowy};
use crab_world::blocks::{init_blocks, GrassBlock};
use crab_world::{BlockType, BLOCKS};

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
struct ComplexBlockData {
    pub boolean: bool,
    pub number: u16,
    pub enum_value: ExampleEnum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(u16)]
enum ExampleEnum {
    A,
    B,
    C,
}

#[test]
fn complex_block_data() {
    let properties = HashMap::from([("boolean", 0), ("number", 3), ("enum_value", 1)]);

    let data: ComplexBlockData = ComplexBlockData::from_properties(properties).unwrap();
    // Note: 0 represents true and 1 represents false.
    assert!(data.boolean);
    assert_eq!(data.number, 3);
    assert_eq!(data.enum_value, ExampleEnum::B);
}

#[test]
fn invalid_block_data() {
    let properties = HashMap::from([("boolean", 3), ("number", 90), ("enum_value", 5)]);

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn missing_block_data() {
    let properties = HashMap::new();

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn get_block_struct_information() {
    BLOCKS.get_or_init(init_blocks);

    let mut block = GrassBlock::default_state_id().unwrap();

    assert!(!block.property::<Snowy>().unwrap());
    block.apply_property::<Snowy>(&true).unwrap();
    assert!(block.property::<Snowy>().unwrap());
}
