use serde::{Deserialize, Serialize};
use std::fs;

use crab_config::configurable::Configurable;
use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;
use crab_config::configuration_entry::ConfigurationEntry;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use crab_macros::Config;

#[derive(Config, Serialize, Deserialize, PartialEq, Debug)]
struct TestConfig {
    /// DOCS TEST
    #[default_value("0")]
    pub version: u8,
    #[default_value("264306")]
    pub test: u32,
}

const TEST_FILE: &str = "check_documentation_persistence.yml";

#[test]
fn check_documentation_persistence() {
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
    let read_result = configurable.read_config().unwrap();
    configurable.write_config(read_result).unwrap();
    let result = configurable.read_config().unwrap();

    let configuration_entry = get_configuration_entry_from_flat_key(&result, "version").unwrap();
    assert_eq!(configuration_entry.documentation.unwrap(), " DOCS TEST");
    let _ = fs::remove_file(TEST_FILE);
}
