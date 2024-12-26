use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use std::fs;

const TEST_FILE: &str = "default_save_test.yml";

#[test]
fn default_save_test() {
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    let test_config = configurable
        .read_and_save_default_if_config_doesnt_exists(Vec::new())
        .unwrap();

    assert_eq!(test_config.test, 264306);
    assert_eq!(test_config.version, 0);

    fs::remove_file(TEST_FILE).unwrap();
}
