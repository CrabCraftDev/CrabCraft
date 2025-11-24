use crate::node::{ConfigurationNode, FLAT_PATH_SEPARATOR, NodeContent};
use serde_json::Value;

impl ConfigurationNode {
    pub fn add_child_by_flat_path(&mut self, flat_path: &str, configuration_node: ConfigurationNode) -> bool {
        if let NodeContent::Value(_) = &mut self.content {
            return false;
        }

        let flat_path = flat_path.split(FLAT_PATH_SEPARATOR).last().unwrap();

        let add_child_by_flat_path = |entry: &mut ConfigurationNode| {
            let NodeContent::Children(children) = &mut entry.content else {
                return;
            };

            children.push((flat_path.to_string(), configuration_node.clone()));
        };
        self.execute_for_configuration_node_in_flat_path(flat_path, &add_child_by_flat_path)
    }

    pub fn add_key_value_by_flat_path(&mut self, flat_path: &str, key: &str, value: Value) -> bool {
        let add_value_by_flat_path = |entry: &mut ConfigurationNode| {
            let NodeContent::Children(children) = &mut entry.content else {
                return;
            };

            children.push((key.to_string(), ConfigurationNode::with_value(value.clone())));
        };
        self.execute_for_configuration_node_in_flat_path(flat_path, &add_value_by_flat_path)
    }

    pub fn add_documentation_by_flat_path(&mut self, flat_path: &str, documentation_value: String) -> bool {
        let set_documentation_by_flat_path = |x: &mut ConfigurationNode| {
            x.documentation = Some(documentation_value.clone());
        };
        self.add_flat_path_if_path_doesnt_exist(flat_path);
        self.execute_for_configuration_node_in_flat_path(flat_path, &set_documentation_by_flat_path)
    }

    /// NOTE: It also creates additional keys with children if flat path doesn't exist
    pub fn add_flat_path_if_path_doesnt_exist(&mut self, flat_path: &str) -> bool {
        let keys: Vec<&str> = flat_path.split(&FLAT_PATH_SEPARATOR).collect();
        self.add_entries_recursively(&keys);
        true
    }

    /// Helper function for recursive traversal and addition of missing entries
    fn add_entries_recursively(&mut self, keys: &[&str]) {
        let Some(key) = keys.first() else { return };

        let mut child_found = false;
        if let NodeContent::Children(children) = &mut self.content {
            // Check if the key exists in the current node's children
            if let Some((_, child)) = children.iter_mut().find(|(local_key, _)| local_key == key) {
                child.add_entries_recursively(&keys[1..]);
                child_found = true;
            }
        }

        if !child_found {
            // If the key doesn't exist, create a new ConfigurationNode
            let new_entry = ConfigurationNode::with_children(Vec::new());

            // Add the new entry to the current node's children
            if let NodeContent::Children(children) = &mut self.content {
                children.push((key.to_string(), new_entry));
            }

            // Move to the newly created child node
            if let NodeContent::Children(children) = &mut self.content {
                let (_, newly_added_child) = children.last_mut().unwrap();
                newly_added_child.add_entries_recursively(&keys[1..]);
            }
        }
    }
}
