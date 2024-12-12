use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_macros::Config;
use serde::{Deserialize, Serialize};
use std::fs;

const TEST_FILE: &str = "save_defaults_read_again.yml";

#[derive(Config, Serialize, Deserialize)]
struct TestConfig {
    /// TEST DOCUMENTAITON 'VERY TESTABLE DOVCUMENITATONI' "SOMETHING SOMETHING"
    #[default_value("0")]
    pub version: u8,
    #[default_value("264306")]
    pub test: u32,
}

#[test]
fn save_defaults_read_again() {
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable
        .read_and_save_default_if_config_doesnt_exists(Vec::new())
        .unwrap();

    let test_config = configurable.read_config_and_verify(Vec::new()).unwrap();

    assert_eq!(test_config.test, 264306);
    assert_eq!(test_config.version, 0);
    fs::remove_file(TEST_FILE).unwrap();
}
