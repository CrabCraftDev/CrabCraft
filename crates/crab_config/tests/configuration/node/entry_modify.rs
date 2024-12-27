use crate::utils::get_test_configuration_node;
use crab_config::node::search::get_configuration_node_from_flat_key;
use serde_yml::Value;

#[test]
fn modify_existing_configuration_node() {
    let mut test_configuration_node = get_test_configuration_node();

    test_configuration_node.set_value_by_flat_path("1234.12345", Value::String("Testing".to_string()));

    let result = get_configuration_node_from_flat_key(&test_configuration_node, "1234.12345");

    let tested_entry = result.unwrap();

    assert_eq!(tested_entry.get_value().unwrap(), "Testing")
}
