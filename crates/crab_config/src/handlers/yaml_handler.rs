use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use regex::Regex;
use serde_yml::{Mapping, Value};

use crate::errors::Error;

use crate::configuration_entry::configuration_entry::ROOT_ENTRY_NAME;
use crate::configuration_entry::ConfigurationEntry;
use crate::handlers::format_handler::ConfigFormatHandler;

pub struct YamlHandler;

const NULL_VALUE_ENUM_DEFAULT: &str = "NULL";

impl YamlHandler {
    pub fn map_values_to_configuration_entry(mapping: &Mapping) -> Result<ConfigurationEntry, Error> {
        let result_of_config_entries = Self::get_configuration_entries_from_mapping(mapping);
        match result_of_config_entries {
            Ok(config_entries) => Ok(ConfigurationEntry::new(
                ROOT_ENTRY_NAME.to_string(),
                Some(config_entries),
                None,
                true,
            )),
            Err(error) => Err(error),
        }
    }

    fn get_configuration_entries_from_mapping(mapping: &Mapping) -> Result<Vec<ConfigurationEntry>, Error> {
        let mut config_entries: Vec<ConfigurationEntry> = Vec::with_capacity(mapping.len());

        for (key, value) in mapping {
            if !key.is_string() {
                continue;
            }
            let key_string = key.as_str().unwrap().to_string();

            if value.is_mapping() {
                let value_mapping = value.as_mapping().unwrap();
                let entries_result = Self::get_configuration_entries_from_mapping(value_mapping);

                match entries_result {
                    Ok(entries) => {
                        config_entries.push(ConfigurationEntry::new(key_string, Some(entries), None, false));
                    }
                    Err(error) => return Err(error),
                }
            } else if value.is_string() || value.is_bool() || value.is_number() || value.is_null() {
                config_entries.push(ConfigurationEntry::new(
                    key_string,
                    None,
                    Some(Self::convert_value_to_string(value)),
                    false,
                ))
            } else {
                return Err(Error::MatchedUnsupportedTypes {
                    is_mapping: value.is_mapping(),
                    is_sequence: value.is_sequence(),
                });
            }
        }
        Ok(config_entries)
    }

    /// It doesn't correctly convert all Value enums
    /// due to how already we convert some values in function `get_configuration_entries_from_mapping`
    /// because they're hierarchical and they cant be easily turned into string
    fn convert_value_to_string(value: &Value) -> String {
        match value {
            Value::Bool(_) => value.as_bool().unwrap().to_string(),
            Value::Number(_) => {
                if value.is_f64() {
                    value.as_f64().unwrap().to_string()
                } else if value.is_i64() {
                    value.as_i64().unwrap().to_string()
                } else {
                    value.as_u64().unwrap().to_string()
                }
            }
            Value::String(_) => value.as_str().unwrap().to_string(),
            Value::Null => NULL_VALUE_ENUM_DEFAULT.to_string(),
            // These cases shouldn't be handled because you can't convert them to string
            Value::Sequence(_) => panic!(
                "UNDEFINED BEHAVIOR Value converted in format handler was Sequence expected String | number | boolean"
            ),
            Value::Mapping(_) => panic!(
                "UNDEFINED BEHAVIOR Value converted in format handler was Mapping expected String | number | boolean"
            ),
            Value::Tagged(_) => panic!(
                "UNDEFINED BEHAVIOR Value converted in format handler was Tagged expected String | number | boolean"
            ),
        }
    }
}

impl ConfigFormatHandler for YamlHandler {
    fn read(&self, file_path: String) -> Result<ConfigurationEntry, Error> {
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(Error::ConfigurationFileNotFound {
                configuration_file_path: file_path,
            });
        }

        let file = OpenOptions::new().read(true).open(&file_path)?;
        let yaml_value: Value = serde_yml::from_reader(file)?;
        let mapping = yaml_value.as_mapping().ok_or(Error::UndefinedBehavior)?;

        Self::map_values_to_configuration_entry(mapping)
    }

    fn write(&self, file_path: String, contents: ConfigurationEntry) -> Result<(), Error> {
        let mut file = File::create(&file_path)?;

        let serialized = serde_yml::to_string(&contents)?;

        // TODO: FIND ANY WORKING LIBRARY WITH COMMENT HANDLING serde_yaml is fake news
        let regex = Regex::new(r#"(comment: ')(.*)('\n)"#).unwrap();
        let modified_content = regex.replace_all(&serialized, "#$2\n");

        file.write_all(modified_content.as_bytes())?;
        Ok(())
    }
}
