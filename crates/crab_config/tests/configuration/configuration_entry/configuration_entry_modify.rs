use crate::configuration::configuration_entry::utils::get_test_configuration_entry;
use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;

#[test]
fn modify_existing_configuration_entry() {
    let mut test_configuration_entry = get_test_configuration_entry();

    test_configuration_entry.set_value_by_flat_path("1234.12345", "Testing".to_string());

    let result = get_configuration_entry_from_flat_key(&test_configuration_entry, "1234.12345");

    let tested_entry = result.unwrap();

    assert_eq!(tested_entry.get_value().unwrap(), "Testing")
}
