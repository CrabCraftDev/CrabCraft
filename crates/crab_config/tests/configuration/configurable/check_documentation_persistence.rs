use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use std::fs;

const TEST_FILE: &str = "check_documentation_persistence.yml";

#[test]
fn check_documentation_persistence() {
    let default = TestConfig::default();
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.write(default.as_configuration_entry()).unwrap();
    let read_result = configurable.read_config().unwrap();
    configurable.write_node(read_result).unwrap();
    let result = configurable.read_config().unwrap();

    let configuration_entry = get_configuration_entry_from_flat_key(&result, "version").unwrap();
    assert_eq!(
        configuration_entry.documentation.unwrap(),
        " First value documentation\n Which has second line :3"
    );
    let configuration_entry = get_configuration_entry_from_flat_key(&result, "test").unwrap();
    assert_eq!(
        configuration_entry.documentation.unwrap(),
        " Second value \"documentation\""
    );

    let _ = fs::remove_file(TEST_FILE);
}
