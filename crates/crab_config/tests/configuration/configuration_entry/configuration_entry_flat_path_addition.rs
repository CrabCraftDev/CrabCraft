use crate::configuration::configuration_entry::utils::get_test_configuration_entry;
use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;

pub const NEWLY_ADDED_KEY: &str = "1234567";

#[test]
fn modify_existing_configuration_entry() {
    let mut test_configuration_entry = get_test_configuration_entry();

    test_configuration_entry.add_flat_path_if_path_doesnt_exist(&("1234.12345.".to_string() + NEWLY_ADDED_KEY));

    let result = get_configuration_entry_from_flat_key(&test_configuration_entry, "1234.12345");

    let tested_entry = result.unwrap();

    let tested_entry_targeted_children = tested_entry.get_next().unwrap();
    let first_child = tested_entry_targeted_children.first();

    assert!(first_child.is_some());
    assert_eq!(first_child.unwrap().key, NEWLY_ADDED_KEY)
}
