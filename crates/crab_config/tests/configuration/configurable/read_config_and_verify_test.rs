use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use semver::Version;
use std::fs;

const TEST_FILE: &str = "read_config_and_verify.yml";

#[test]
fn read_config_and_verify() {
    let default = TestConfig::default();
    let configurable = Configurable::<TestConfig, Version>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.write_node(default.as_configuration_node()).unwrap();
    let result = configurable.read_config_and_update(Vec::new()).unwrap();
    assert_eq!(result, default);
    fs::remove_file(TEST_FILE).unwrap();
}
