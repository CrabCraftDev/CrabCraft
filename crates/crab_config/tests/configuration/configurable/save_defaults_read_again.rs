use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use semver::Version;
use std::fs;

const TEST_FILE: &str = "save_defaults_read_again.yml";

#[test]
fn save_defaults_read_again() {
    let configurable = Configurable::<TestConfig, Version>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.load_or_create_default_config(Vec::new()).unwrap();

    let test_config = configurable.read_config_and_update(Vec::new()).unwrap();

    assert_eq!(test_config.test, 264306);
    assert_eq!(test_config._version, "0.0.2");
    fs::remove_file(TEST_FILE).unwrap();
}
