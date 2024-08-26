use std::collections::HashMap;

use strum::FromRepr;

use crab_macros::BlockData;
use crab_world::block_data::BlockData;

#[derive(Debug, Clone, PartialEq, Eq, BlockData)]
struct ComplexBlockData {
    pub boolean: bool,
    pub number: u16,
    pub enum_value: ExampleEnum,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRepr)]
#[repr(u16)]
enum ExampleEnum {
    A,
    B,
    C,
}

#[test]
fn complex_block_data() {
    let mut properties = HashMap::new();
    properties.insert("boolean".to_string(), 0);
    properties.insert("number".to_string(), 3);
    properties.insert("enum_value".to_string(), 1);

    let data: ComplexBlockData = ComplexBlockData::from_properties(properties).unwrap();
    assert!(data.boolean);
    assert_eq!(data.number, 3);
    assert_eq!(data.enum_value, ExampleEnum::B);
}

#[test]
fn invalid_block_data() {
    let mut properties = HashMap::new();
    properties.insert("boolean".to_string(), 3);
    properties.insert("number".to_string(), 90);
    properties.insert("enum_value".to_string(), 5);

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn missing_block_data() {
    let properties = HashMap::new();

    assert!(ComplexBlockData::from_properties(properties).is_err());
}
