use crate::configuration_entry::configuration_entry::{FLAT_PATH_SEPARATOR, ROOT_ENTRY_NAME};
use crate::configuration_entry::ConfigurationEntry;
use crate::errors::Error;
use std::collections::HashMap;

impl ConfigurationEntry {
    /// Converts Tree Structure to flattened hashmap <br />
    /// Example: <br />
    /// ```none
    ///   root
    /// |      \
    /// test  test2
    /// |
    /// test3
    /// ```
    /// Gives out following hashmap with keys: <br />
    /// test.test3 <br />
    /// test2 <br />
    pub fn serialize_to_flatten_map_with_path(&self) -> Result<HashMap<String, String>, Error> {
        let separator = FLAT_PATH_SEPARATOR.to_string();

        if !self.does_next_exist() {
            let mut map: HashMap<String, String> = HashMap::new();
            let Some(current_value) = self.get_value() else {
                return Ok(map);
            };
            map.insert(self.key.clone(), current_value.clone());
            return Ok(map);
        }

        let array = self.get_next()?;

        Self::flatten_entries_in_array(&separator, &self.key, array)
    }

    fn flatten_entries_in_array(
        separator: &String,
        parent_key: &String,
        array: &Vec<ConfigurationEntry>,
    ) -> Result<HashMap<String, String>, Error> {
        let mut map: HashMap<String, String> = HashMap::new();

        for element in array {
            let flatten_map = element.serialize_to_flatten_map_with_path()?;
            for (key, value) in flatten_map {
                map.insert(
                    Self::format_flatten_entry_key(parent_key, &key, separator),
                    value.clone(),
                );
            }
        }

        Ok(map)
    }

    fn format_flatten_entry_key(parent_key: &String, key: &String, separator: &String) -> String {
        if parent_key != ROOT_ENTRY_NAME {
            format!("{}{}{}", parent_key, separator, key)
        } else {
            key.to_string()
        }
    }

    pub fn copy_docs_from(&mut self, other: &ConfigurationEntry) {
        // Check if both have the same key to ensure matching structures
        if self.key != other.key {
            return;
        }

        // If `self` lacks documentation, copy from `other`
        if self.documentation.is_none() {
            self.documentation.clone_from(&other.documentation);
        }

        // If both have children, handle recursively with key-based lookup
        let (Some(self_children), Some(other_children)) = (&mut self.children, &other.children) else {
            return;
        };
        for self_child in self_children.iter_mut() {
            // Find matching child in `other` by key
            if let Some(other_child) = other_children.iter().find(|c| c.key == self_child.key) {
                self_child.copy_docs_from(other_child);
            }
        }
    }
}
