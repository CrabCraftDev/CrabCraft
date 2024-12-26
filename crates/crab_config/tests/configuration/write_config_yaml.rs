use std::fs::{File, OpenOptions, remove_file};
use std::path::Path;

use serde_yml::{Mapping, Number, Value};

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

    let test_value_1 = Value::String("Test1".to_owned());
    let test_value_2 = Value::String("Test2".to_owned());
    let test_value_3 = Value::Number(Number::from(2137));
    let test_value_4 = Value::Number(Number::from(69.42));

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure>::new(TEST_FILE.to_string(), Box::new(yaml_handler));

    let children = vec![
        ConfigurationEntry::new("TEST_VALUE_1".to_string(), None, Some(test_value_1.clone()), false),
        ConfigurationEntry::new("TEST_VALUE_2".to_string(), None, Some(test_value_2.clone()), false),
        ConfigurationEntry::new("TEST_VALUE_3".to_string(), None, Some(test_value_3.clone()), false),
        ConfigurationEntry::new(
            "TEST_STRUCTURE_1".to_string(),
            Some(
                [ConfigurationEntry::new(
                    "TEST_VALUE_4".to_string(),
                    None,
                    Some(test_value_4.clone()),
                    false,
                )]
                .to_vec(),
            ),
            None,
            false,
        ),
    ];

    configurable.write(children).unwrap();

    let result = serde_yml::from_reader::<File, Mapping>(OpenOptions::new().read(true).open(TEST_FILE).unwrap());
    let value = result.unwrap();
    assert_eq!(value["TEST_VALUE_1"], test_value_1);
    assert_eq!(value["TEST_VALUE_2"], test_value_2);
    assert_eq!(value["TEST_VALUE_3"], test_value_3);
    assert_eq!(
        value["TEST_STRUCTURE_1"].as_mapping().unwrap()["TEST_VALUE_4"],
        test_value_4
    );

    remove_file(TEST_FILE).unwrap();
}
