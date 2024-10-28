use crab_config::configurable::Configurable;
use crab_config::configuration_entry::ConfigurationEntry;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use crab_macros::Config;
use serde::{Deserialize, Serialize};
use std::fs;

const TEST_FILE: &str = "read_config_and_verify.yml";

#[derive(Config, Serialize, Deserialize, PartialEq, Debug)]
struct TestConfig {
    /// TEST DOCUMENTAITON 'VERY TESTABLE DOVCUMENITATONI' "SOMETHING SOMETHING"
    #[default_value("0")]
    pub version: u8,
    #[default_value("264306")]
    pub test: u32,
}

#[test]
fn read_config_and_verify() {
    let default = TestConfig::default();
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable
        .write_config(ConfigurationEntry::new(
            "root".to_string(),
            Some(default.map_to_configuration_entry()),
            None,
            true,
        ))
        .unwrap();
    let result = configurable.read_config_and_verify(Vec::new()).unwrap();
    assert_eq!(result, default);
    fs::remove_file(TEST_FILE).unwrap();
}
