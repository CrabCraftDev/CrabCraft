use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use std::fs;

const TEST_FILE: &str = "read_config_and_verify.yml";

#[test]
fn read_config_and_verify() {
    let default = TestConfig::default();
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.write(default.as_configuration_entry()).unwrap();
    let result = configurable.read_config_and_verify(Vec::new()).unwrap();
    assert_eq!(result, default);
    fs::remove_file(TEST_FILE).unwrap();
}
