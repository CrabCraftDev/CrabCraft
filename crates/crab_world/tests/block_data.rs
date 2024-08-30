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
    let properties: HashMap<String, u16> = HashMap::from([
        ("boolean".to_owned(), 0),
        ("number".to_owned(), 3),
        ("enum_value".to_owned(), 1),
    ]);

    let data: ComplexBlockData = ComplexBlockData::from_properties(properties).unwrap();
    // Note: 0 represents true and 1 represents false.
    assert!(data.boolean);
    assert_eq!(data.number, 3);
    assert_eq!(data.enum_value, ExampleEnum::B);
}

#[test]
fn invalid_block_data() {
    let properties = HashMap::from([
        ("boolean".to_owned(), 3),
        ("number".to_owned(), 90),
        ("enum_value".to_owned(), 5),
    ]);

    assert!(ComplexBlockData::from_properties(properties).is_err());
}

#[test]
fn missing_block_data() {
    let properties = HashMap::new();

    assert!(ComplexBlockData::from_properties(properties).is_err());
}
