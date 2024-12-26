use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use regex::Regex;
use serde_yml::{Mapping, Value};

use crate::errors::Error;

use crate::configuration_entry::ConfigurationEntry;
use crate::configuration_entry::configuration_entry::ROOT_ENTRY_NAME;
use crate::handlers::format_handler::ConfigFormatHandler;

pub struct YamlHandler;

impl YamlHandler {
    pub fn map_values_to_configuration_entry(mapping: &Mapping) -> Result<ConfigurationEntry, Error> {
        let config_entries = Self::get_configuration_entries_from_mapping(mapping)?;
        Ok(ConfigurationEntry::new(
            ROOT_ENTRY_NAME.to_string(),
            Some(config_entries),
            None,
            true,
        ))
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
            } else {
                config_entries.push(ConfigurationEntry::new(key_string, None, Some(value.clone()), false));
            }
        }
        Ok(config_entries)
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
