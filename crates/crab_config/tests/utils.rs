use crab_config::node::ConfigurationNode;
use crab_config::types::config::Config;
use crab_macros::Config;
use serde::{Deserialize, Serialize};
use serde_yml::Value;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Config, Serialize, Deserialize, PartialEq, Debug)]
pub struct TestConfig {
    /// First value documentation
    /// Which has second line :3
    #[default_value("0.0.2".to_string())]
    pub _version: String,
    /// Second value "documentation"
    #[default_value(264306)]
    pub test: u32,
}

pub fn get_test_configuration_node() -> ConfigurationNode {
    ConfigurationNode::with_children(Vec::from_iter([
        (
            "1".to_string(),
            ConfigurationNode::with_value(Value::String("Test".to_string())),
        ),
        (
            "12".to_string(),
            ConfigurationNode::with_value(Value::String("Test".to_string())),
        ),
        (
            "123".to_string(),
            ConfigurationNode::with_value(Value::String("Test".to_string())),
        ),
        (
            "1234".to_string(),
            ConfigurationNode::with_children(Vec::from_iter([(
                "12345".to_string(),
                ConfigurationNode::with_value(Value::String("Test".to_string())),
            )])),
        ),
    ]))
}

pub fn create_test_file(test_structure: impl Config + Serialize, file: &str) {
    let path = Path::new(file);
    if path.exists() {
        return;
    }

    let file = OpenOptions::new().create_new(true).append(true).open(file).unwrap();

    serde_yml::to_writer(file, &test_structure).unwrap();
}

pub fn remove_test_file(file: &str) {
    fs::remove_file(file).unwrap();
}
