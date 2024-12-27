use crate::utils::get_test_configuration_node;
use crab_config::node::search::get_configuration_node_from_flat_key;

pub const NEWLY_ADDED_KEY: &str = "1234567";

#[test]
fn add_new_configuration_node() {
    let mut test_configuration_node = get_test_configuration_node();

    test_configuration_node.add_flat_path_if_path_doesnt_exist(&("1234.12346.".to_string() + NEWLY_ADDED_KEY));

    let result = get_configuration_node_from_flat_key(&test_configuration_node, "1234.12346");

    let tested_entry = result.unwrap();
    let tested_entry_targeted_children = tested_entry.get_children().unwrap();
    let first_child = tested_entry_targeted_children.first();

    assert!(first_child.is_some());
    assert_eq!(first_child.unwrap().0, NEWLY_ADDED_KEY);
}

#[test]
fn modify_existing_configuration_node() {
    let mut test_configuration_node = get_test_configuration_node();

    test_configuration_node.add_flat_path_if_path_doesnt_exist(&("1234.12345.".to_string() + NEWLY_ADDED_KEY));

    let result = get_configuration_node_from_flat_key(&test_configuration_node, "1234.12345");

    let tested_entry = result.unwrap();
    assert_eq!(tested_entry.get_children(), None);
    assert!(tested_entry.get_value().is_some());
}
