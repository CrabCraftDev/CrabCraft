use std::fs::{remove_file, File, OpenOptions};
use std::path::Path;

use serde_yml::Mapping;

use crate::configuration::read_config_yaml::TestStructure;
use crab_config::configurable::Configurable;
use crab_config::configuration_entry::ConfigurationEntry;
use crab_config::handlers::yaml_handler::YamlHandler;

const TEST_FILE: &str = "write_config_yaml.yml";

#[test]
fn write_config_yaml_from_flattened_map() {
    // Setup

    let path = Path::new(TEST_FILE);
    if path.exists() {
        remove_file(path).unwrap();
    }

    const TEST_VALUE_1: &str = "Test1";

    const TEST_VALUE_2: &str = "Test2";

    const TEST_VALUE_3: i32 = 2137;

    const TEST_VALUE_4: f32 = 2.137;

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure>::new(TEST_FILE.to_string(), Box::new(yaml_handler));

    let children = Some(
        [
            ConfigurationEntry::new("TEST_VALUE_1".to_string(), None, Some(TEST_VALUE_1.to_string()), false),
            ConfigurationEntry::new("TEST_VALUE_2".to_string(), None, Some(TEST_VALUE_2.to_string()), false),
            ConfigurationEntry::new("TEST_VALUE_3".to_string(), None, Some(TEST_VALUE_3.to_string()), false),
            ConfigurationEntry::new(
                "TEST_STRUCTURE_1".to_string(),
                Some(
                    [ConfigurationEntry::new(
                        "TEST_VALUE_4".to_string(),
                        None,
                        Some(TEST_VALUE_4.to_string()),
                        false,
                    )]
                    .to_vec(),
                ),
                None,
                false,
            ),
        ]
        .to_vec(),
    );

    let configuration_entry = ConfigurationEntry::new("root".to_string(), children, None, true);

    configurable.write_config(configuration_entry).unwrap();

    let result = serde_yml::from_reader::<File, Mapping>(OpenOptions::new().read(true).open(TEST_FILE).unwrap());
    let value = result.unwrap();
    assert_eq!(value["TEST_VALUE_1"].as_str().unwrap(), TEST_VALUE_1.to_string());
    assert_eq!(value["TEST_VALUE_2"].as_str().unwrap(), TEST_VALUE_2.to_string());
    assert_eq!(value["TEST_VALUE_3"].as_str().unwrap(), TEST_VALUE_3.to_string());
    assert_eq!(
        value["TEST_STRUCTURE_1"].as_mapping().unwrap()["TEST_VALUE_4"]
            .as_str()
            .unwrap(),
        TEST_VALUE_4.to_string()
    );

    remove_file(TEST_FILE).unwrap();
}
