use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use regex::Regex;
use serde_json::{Map, Value};

use crate::error::ConfigError;

use crate::handlers::format_handler::ConfigFormatHandler;
use crate::node::ConfigurationNode;

pub struct YamlHandler;

impl YamlHandler {
    pub fn map_values_to_configuration_node(mapping: &Map<String, Value>) -> ConfigurationNode {
        let config_tree = Self::get_configuration_tree_from_mapping(mapping);
        ConfigurationNode::with_children_as_hashmap(config_tree)
    }

    fn get_configuration_tree_from_mapping(mapping: &Map<String, Value>) -> HashMap<String, ConfigurationNode> {
        let mut config_tree = HashMap::with_capacity(mapping.len());

        for (key, value) in mapping {
            if value.is_object() {
                let value_mapping = value.as_object().unwrap();
                let entries = Self::get_configuration_tree_from_mapping(value_mapping);

                config_tree.insert(key.to_string(), ConfigurationNode::with_children_as_hashmap(entries));
            } else {
                config_tree.insert(key.to_string(), ConfigurationNode::with_value(value.clone()));
            }
        }

        config_tree
    }
}

impl ConfigFormatHandler for YamlHandler {
    fn read(&self, file_path: String) -> Result<ConfigurationNode, ConfigError> {
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(ConfigError::ConfigurationFileNotFound {
                configuration_file_path: file_path,
            });
        }

        let file = OpenOptions::new().read(true).open(&file_path)?;
        let yaml_value: Value = serde_saphyr::from_reader(file)?;
        let mapping = yaml_value.as_object().ok_or(ConfigError::InvalidConfigSyntax)?;

        Ok(Self::map_values_to_configuration_node(mapping))
    }

    fn write(&self, file_path: String, contents: ConfigurationNode) -> Result<(), ConfigError> {
        let mut file = File::create(&file_path)?;

        let serialized = serde_saphyr::to_string(&contents)?;

        let regex = Regex::new(r#"(text-to-insert: ["'])(.*)(["']\n)"#).unwrap();
        let modified_content = regex.replace_all(&serialized, "$2\n");

        file.write_all(modified_content.as_bytes())?;
        Ok(())
    }
}
