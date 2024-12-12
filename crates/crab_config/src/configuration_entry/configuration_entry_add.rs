use crate::configuration_entry::configuration_entry::FLAT_PATH_SEPARATOR;
use crate::configuration_entry::ConfigurationEntry;

impl ConfigurationEntry {
    // Useful method in testing
    #[allow(dead_code)]
    fn add_child_by_flat_path(&mut self, flat_path: &str, configuration_entry: ConfigurationEntry) -> bool {
        if self.value.is_some() {
            return false;
        }

        let add_child_by_flat_path = |entry: &mut ConfigurationEntry| {
            if entry.children.is_none() {
                return;
            }
            let mut children = entry.get_next().cloned().unwrap_or_default();

            children.push(configuration_entry.clone());

            entry.children = Some(children);
        };
        self.execute_for_configuration_entry_in_flat_path(flat_path, &add_child_by_flat_path)
    }

    pub fn add_key_value_by_flat_path(&mut self, flat_path: &str, key: &str, value: String) -> bool {
        let add_value_by_flat_path = |x: &mut ConfigurationEntry| {
            if x.children.is_none() {
                return;
            }
            let next_children_result = x.get_next();
            let mut children = next_children_result.cloned().unwrap_or_else(|_| Vec::new());

            children.push(ConfigurationEntry::new_value(key.to_string(), value.clone()));

            x.children = Some(children);
        };
        self.execute_for_configuration_entry_in_flat_path(flat_path, &add_value_by_flat_path)
    }

    pub fn add_documentation_by_flat_path(&mut self, flat_path: &str, documentation_value: String) -> bool {
        let set_documentation_by_flat_path = |x: &mut ConfigurationEntry| {
            x.documentation = Some(documentation_value.clone());
        };
        self.add_flat_path_if_path_doesnt_exist(flat_path);
        self.execute_for_configuration_entry_in_flat_path(flat_path, &set_documentation_by_flat_path)
    }

    /// NOTE: It also creates additional keys with children if flat path doesn't exist
    pub fn add_flat_path_if_path_doesnt_exist(&mut self, flat_path: &str) -> bool {
        if !self.is_root {
            return false;
        }
        let keys: Vec<&str> = flat_path.split(&FLAT_PATH_SEPARATOR).collect();
        self.add_entries_recursively(&keys);
        true
    }

    /// Helper function for recursive traversal and addition of missing entries
    fn add_entries_recursively(&mut self, keys: &[&str]) {
        let Some(key) = keys.first() else { return };

        let mut child_found = false;
        if let Some(ref mut children) = self.children {
            // Check if the key exists in the current node's children
            if let Some(index) = children.iter().position(|entry| entry.key == *key) {
                let child = &mut children[index];
                // Move to the child node
                child.add_entries_recursively(&keys[1..]);
                child_found = true;
            }
        }

        if !child_found {
            // If the key doesn't exist, create a new ConfigurationEntry object
            let new_entry = ConfigurationEntry {
                key: (*key).to_string(),
                children: None,
                value: None,
                documentation: None,
                is_root: false,
            };

            // Add the new entry to the current node's children
            let mut new_children = self.children.take().unwrap_or_default();
            new_children.push(new_entry);
            self.children = Some(new_children);

            // Move to the newly created child node
            if let Some(ref mut children) = self.children {
                let index = children.len() - 1;
                let child = &mut children[index];
                child.add_entries_recursively(&keys[1..]);
            }
        }
    }
}
