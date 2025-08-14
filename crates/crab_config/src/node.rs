use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_yml::Value;
use std::collections::HashMap;

pub mod add;
pub mod misc;
mod remove;
pub mod search;

pub const FLAT_PATH_SEPARATOR: &str = ".";

/// Represents a single node in configuration tree
/// Node can either have:
///  - children - object containing other nodes
///  - value - serde_yml value containing actual data
///
/// It also stores additional metadata like documentation
#[derive(PartialEq, Debug, Clone, Default)]
pub struct ConfigurationNode {
    pub documentation: Option<String>,
    pub content: NodeContent,
}

#[derive(PartialEq, Debug, Clone)]
pub enum NodeContent {
    Value(Value),
    // We are using vec to preserve order in text configs
    Children(Vec<(String, ConfigurationNode)>),
}

impl Default for NodeContent {
    fn default() -> Self {
        NodeContent::Children(Vec::new())
    }
}

impl ConfigurationNode {
    pub fn new(content: NodeContent) -> ConfigurationNode {
        Self::new_with_doc(content, None)
    }

    pub fn new_with_doc(content: NodeContent, documentation: Option<String>) -> ConfigurationNode {
        ConfigurationNode { content, documentation }
    }

    pub fn set_doc(self, doc: Option<String>) -> ConfigurationNode {
        ConfigurationNode {
            documentation: doc,
            ..self
        }
    }

    pub fn add_doc(mut self, doc: Option<String>) -> ConfigurationNode {
        if let Some(new_doc) = doc {
            if let Some(existing_doc) = &mut self.documentation {
                let mut combined_doc = new_doc;
                combined_doc.push('\n');
                combined_doc.push_str(existing_doc);
                self.documentation = Some(combined_doc);
            } else {
                self.documentation = Some(new_doc);
            }
        }
        self
    }

    pub fn with_value(value: Value) -> ConfigurationNode {
        Self::new(NodeContent::Value(value))
    }

    pub fn with_children(children: Vec<(String, ConfigurationNode)>) -> ConfigurationNode {
        Self::new(NodeContent::Children(children))
    }

    pub fn with_children_as_hashmap(children: HashMap<String, ConfigurationNode>) -> ConfigurationNode {
        Self::with_children(children.into_iter().collect())
    }

    pub fn get_value(&self) -> Option<&Value> {
        match &self.content {
            NodeContent::Value(value) => Some(value),
            NodeContent::Children(_) => None,
        }
    }

    pub fn get_children(&self) -> Option<&Vec<(String, ConfigurationNode)>> {
        match &self.content {
            NodeContent::Value(_) => None,
            NodeContent::Children(children) => Some(children),
        }
    }

    pub fn set_value_by_flat_path(&mut self, flat_path: &str, new_value: Value) -> bool {
        self.add_flat_path_if_path_doesnt_exist(flat_path);
        let set_value_function = |x: &mut ConfigurationNode| {
            x.content = NodeContent::Value(new_value.clone());
        };
        self.execute_for_configuration_node_in_flat_path(flat_path, &set_value_function)
    }

    /// This recursive function searches for `ConfigurationNode` matching the `flat_path` \
    /// it will return either `false` or `true` based off if the `ConfigurationNode` was found \
    /// in the tree and will also execute `function` argument if it has found correct `ConfigurationNode`
    pub fn execute_for_configuration_node_in_flat_path(
        &mut self,
        flat_path: &str,
        function: &dyn Fn(&mut ConfigurationNode),
    ) -> bool {
        let parts: Vec<&str> = flat_path.split(FLAT_PATH_SEPARATOR).collect();
        let NodeContent::Children(children) = &mut self.content else {
            return false;
        };

        let mut new_parts = parts.clone();
        new_parts.remove(0);
        let new_parts = new_parts.join(FLAT_PATH_SEPARATOR);

        for (key, child) in children {
            if key == parts.first().unwrap() {
                if parts.len() == 1 {
                    function(child);
                    return true;
                }

                let res = child.execute_for_configuration_node_in_flat_path(&new_parts, function);
                if res {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_value_from_flat_path(&self, flat_path: &str) -> Option<&Value> {
        let mut parts: Vec<&str> = flat_path.split(FLAT_PATH_SEPARATOR).collect();
        let first_part = parts.first().unwrap();

        if parts.len() == 1 && self.get_children().is_none() {
            return self.get_value();
        }

        if let NodeContent::Children(children) = &self.content {
            for (key, child) in children {
                if key == *first_part {
                    if parts.len() == 1 {
                        return child.get_value();
                    }
                    parts.remove(0);
                    let next_parts = parts.join(FLAT_PATH_SEPARATOR);
                    return child.get_value_from_flat_path(&next_parts);
                }
            }
        }

        None
    }

    pub fn merge(&mut self, user: &ConfigurationNode) {
        if let NodeContent::Children(user_children) = &user.content {
            if let NodeContent::Children(self_children) = &mut self.content {
                for (user_key, user_child) in user_children {
                    if let Some((_, self_child)) = self_children.iter_mut().find(|(key, _)| key == user_key) {
                        self_child.merge(user_child);
                    }
                }
            } else {
                self.content = NodeContent::Children(user_children.clone());
            }
        }

        if let NodeContent::Value(user_value) = &user.content {
            self.content = NodeContent::Value(user_value.clone());
        }
    }
}

impl Serialize for ConfigurationNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let children = match &self.content {
            NodeContent::Value(value) => return value.serialize(serializer),
            NodeContent::Children(children) => children,
        };

        let mut serialized_struct = serializer.serialize_map(Some(children.len()))?;
        for (i, (key, child)) in children.iter().enumerate() {
            if let Some(documentation) = &child.documentation {
                if i > 1 {
                    serialized_struct.serialize_entry("text-to-insert", "")?;
                }
                for line in documentation.lines() {
                    serialized_struct.serialize_entry("text-to-insert", &format!("#{line}"))?;
                }
            }

            match &child.content {
                NodeContent::Value(value) => serialized_struct.serialize_entry(&key, value)?,
                NodeContent::Children(_) => serialized_struct.serialize_entry(&key, &child)?,
            }
        }

        serialized_struct.end()
    }
}
