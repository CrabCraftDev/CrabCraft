use crate::configuration::read_config_yaml::TestStructure;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::node::ConfigurationNode;
use semver::Version;
use serde_yml::{Mapping, Number, Value};
use std::fs::{File, OpenOptions, remove_file};
use std::path::Path;

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
    let configurable = Configurable::<TestStructure, Version>::new(TEST_FILE.to_string(), Box::new(yaml_handler));

    let config = ConfigurationNode::with_children(Vec::from_iter([
        (
            "TEST_VALUE_1".to_string(),
            ConfigurationNode::with_value(test_value_1.clone()),
        ),
        (
            "TEST_VALUE_2".to_string(),
            ConfigurationNode::with_value(test_value_2.clone()),
        ),
        (
            "TEST_VALUE_3".to_string(),
            ConfigurationNode::with_value(test_value_3.clone()),
        ),
        (
            "TEST_STRUCTURE_1".to_string(),
            ConfigurationNode::with_children(Vec::from_iter([(
                "TEST_VALUE_4".to_string(),
                ConfigurationNode::with_value(test_value_4.clone()),
            )])),
        ),
    ]));

    configurable.write_node(config).unwrap();

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
