use crate::error::ConfigError;
use crate::node::{ConfigurationNode, FLAT_PATH_SEPARATOR, NodeContent};
use serde_yml::Value;
use std::collections::HashMap;

impl ConfigurationNode {
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
    pub fn serialize_to_flatten_map_with_path(&self) -> Result<HashMap<String, Value>, ConfigError> {
        Ok(self.serialize_to_flatten_map_with_path_internal(String::new()))
    }

    fn serialize_to_flatten_map_with_path_internal(&self, prefix: String) -> HashMap<String, Value> {
        let mut flat_map = HashMap::new();

        if let NodeContent::Value(value) = &self.content {
            flat_map.insert(prefix.clone(), value.clone());
        }

        if let NodeContent::Children(children) = &self.content {
            for (child_name, child_node) in children {
                let mut child_prefix = prefix.clone();
                if !child_prefix.is_empty() {
                    child_prefix.push_str(FLAT_PATH_SEPARATOR);
                }
                child_prefix.push_str(child_name);

                let child_flat = child_node.serialize_to_flatten_map_with_path_internal(child_prefix);
                flat_map.extend(child_flat);
            }
        }

        flat_map
    }

    pub fn copy_docs_from(&mut self, other: &ConfigurationNode) {
        if self.documentation.is_none() {
            self.documentation.clone_from(&other.documentation);
        }

        let (NodeContent::Children(self_children), NodeContent::Children(other_children)) =
            (&mut self.content, &other.content)
        else {
            return;
        };
        for (self_key, child) in self_children.iter_mut() {
            if let Some((_, other_child)) = other_children.iter().find(|(key, _)| key == self_key) {
                child.copy_docs_from(other_child);
            }
        }
    }
}
