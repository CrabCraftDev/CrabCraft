use crab_config::configuration_entry::ConfigurationEntry;
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
    #[default_value(0)]
    pub version: u8,
    /// Second value "documentation"
    #[default_value(264306)]
    pub test: u32,
}

pub fn get_test_configuration_entry() -> ConfigurationEntry {
    ConfigurationEntry::new(
        "root".to_string(),
        Some(
            [
                ConfigurationEntry::new("1".to_string(), None, Some(Value::String("Test".to_string())), false),
                ConfigurationEntry::new("12".to_string(), None, Some(Value::String("Test".to_string())), false),
                ConfigurationEntry::new("123".to_string(), None, Some(Value::String("Test".to_string())), false),
                ConfigurationEntry::new(
                    "1234".to_string(),
                    Some(
                        [ConfigurationEntry::new(
                            "12345".to_string(),
                            None,
                            Some(Value::String("Test".to_string())),
                            false,
                        )]
                        .to_vec(),
                    ),
                    None,
                    false,
                ),
            ]
            .to_vec(),
        ),
        None,
        true,
    )
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
