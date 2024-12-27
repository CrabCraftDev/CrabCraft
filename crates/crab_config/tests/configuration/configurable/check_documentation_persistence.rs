use crate::utils::TestConfig;
use crab_config::configurable::{Configurable, VERSION_CONFIG_FIELD_NAME};
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::node::search::get_configuration_node_from_flat_key;
use crab_config::types::config::Config;
use semver::Version;
use std::fs;

const TEST_FILE: &str = "check_documentation_persistence.yml";

#[test]
fn check_documentation_persistence() {
    let default = TestConfig::default();
    let configurable = Configurable::<TestConfig, Version>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.write_node(default.as_configuration_node()).unwrap();

    let read_result = configurable.read_config().unwrap();
    configurable.write_node(read_result).unwrap();
    let result = configurable.read_config().unwrap();

    let configuration_node = get_configuration_node_from_flat_key(&result, VERSION_CONFIG_FIELD_NAME).unwrap();
    assert_eq!(
        configuration_node.documentation.unwrap(),
        " First value documentation\n Which has second line :3\n Default value: \"0.0.2\""
    );
    let configuration_node = get_configuration_node_from_flat_key(&result, "test").unwrap();
    assert_eq!(
        configuration_node.documentation.unwrap(),
        " Second value \"documentation\"\n Default value: 264306"
    );

    let _ = fs::remove_file(TEST_FILE);
}
