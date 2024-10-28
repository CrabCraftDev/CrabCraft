use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use crate::configuration_entry::ConfigurationEntry;

use crate::errors::Error;

pub const ERROR_KEY: &str = "NULL";
pub const FLAT_PATH_SEPARATOR: &str = ".";
pub const EMPTY_STRING: &str = "";
pub const ROOT_ENTRY_NAME: &str = "root";

impl ConfigurationEntry {
    pub fn new(
        key: String,
        children: Option<Vec<ConfigurationEntry>>,
        value: Option<String>,
        is_root: bool,
    ) -> ConfigurationEntry {
        Self::new_with_doc(key, children, value, None, is_root).unwrap()
    }

    pub fn new_with_doc(
        key: String,
        children: Option<Vec<ConfigurationEntry>>,
        value: Option<String>,
        documentation: Option<String>,
        is_root: bool,
    ) -> Result<ConfigurationEntry, Error> {
        if key.is_empty() {
            return Err(Error::EmptyKey);
        }
        if is_root && value.is_some() {
            return Err(Error::RootCanNotHaveValue);
        }
        if value.is_some() && children.is_some() {
            return Err(Error::NodeCanNotHaveValue);
        }

        Ok(ConfigurationEntry {
            key,
            children,
            value,
            is_root,
            documentation,
        })
    }

    pub fn new_value(key: String, value: String) -> ConfigurationEntry {
        ConfigurationEntry {
            key,
            children: None,
            value: Some(value),
            is_root: false,
            documentation: None,
        }
    }

    pub fn get_next(&self) -> Result<&Vec<ConfigurationEntry>, Error> {
        if let Some(children) = &self.children {
            Ok(children)
        } else {
            Err(Error::NextValueIsEmpty)
        }
    }

    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn does_next_exist(&self) -> bool {
        self.children.is_some()
    }

    pub fn does_value_exist(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn set_value(&mut self, value: String) {
        if self.is_root || self.children.is_some() {
            return;
        }

        self.value = Some(value);
    }

    pub fn set_value_by_flat_path(&mut self, flat_path: &str, new_value: String) -> bool {
        self.add_flat_path_if_path_doesnt_exist(flat_path);
        let set_value_function = |x: &mut ConfigurationEntry| {
            if x.children.is_some() {
                return;
            }
            x.value = Some(new_value.clone());
        };
        self.execute_for_configuration_entry_in_flat_path(flat_path, &set_value_function)
    }

    /// This recursive function searches for `ConfigurationEntry` matching the `flat_path` \
    /// it will return either `false` or `true` based off if the `ConfigurationEntry` was found \
    /// in the tree and will also execute `function` argument if it has found correct `ConfigurationEntry`
    pub fn execute_for_configuration_entry_in_flat_path(
        &mut self,
        flat_path: &str,
        function: &dyn Fn(&mut ConfigurationEntry),
    ) -> bool {
        // Nesting cannot be reduced
        if self.is_root {
            if let Some(children) = &mut self.children {
                for child in children {
                    if child.execute_for_configuration_entry_in_flat_path(flat_path, function) {
                        return true;
                    }
                }
            }
            return false;
        }

        let mut parts: Vec<&str> = flat_path.split(FLAT_PATH_SEPARATOR).collect();

        if let Some(first_part) = parts.first() {
            if *first_part != self.key {
                return false;
            }
            if parts.len() == 1 {
                function(self);
                return true;
            }
            parts.remove(0);
            if let Some(children) = &mut self.children {
                for child in children {
                    let path_suffix = parts.join(FLAT_PATH_SEPARATOR);
                    if child.execute_for_configuration_entry_in_flat_path(&path_suffix, function) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_value_from_flat_path(&self, flat_path: &str) -> Option<&String> {
        let mut parts: Vec<&str> = flat_path.split(&FLAT_PATH_SEPARATOR.to_string()).collect();
        let first_part = parts.first().unwrap().to_string();

        if parts.len() == 1 && first_part == self.key && self.children.is_none() {
            return self.get_value();
        }

        self.children.as_ref()?;

        let result_children = self.get_next();

        let children = result_children.unwrap();

        for child in children {
            if child.key != first_part {
                continue;
            }
            // Handling root node case
            if parts.len() == 1 {
                return child.get_value();
            }
            parts.remove(0);
            let next_parts = parts.join(FLAT_PATH_SEPARATOR);
            return self.get_value_from_flat_path(&next_parts);
        }
        None
    }
}

impl Serialize for ConfigurationEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let children = self.get_next().unwrap();

        let mut serialized_struct = serializer.serialize_map(Some(children.len()))?;
        for child in children {
            let key = child.key.clone();

            let maybe_documentation = &child.documentation.clone();
            if maybe_documentation.is_some() {
                let documentation = maybe_documentation.as_ref().unwrap();
                // TODO: FIND ANY LIBRARY THAT ACCEPTS COMMENTS NATIVELY serde_yaml is fake news
                serialized_struct.serialize_entry("comment", &documentation)?;
            }

            if let Some(value) = &child.value {
                serialized_struct.serialize_entry(&key, value)?;
            } else {
                serialized_struct.serialize_entry(&key, &child)?;
            }
        }

        serialized_struct.end()
    }
}

impl Default for ConfigurationEntry {
    fn default() -> Self {
        ConfigurationEntry {
            key: String::from(ERROR_KEY),
            children: None,
            value: None,
            is_root: false,
            documentation: None,
        }
    }
}
