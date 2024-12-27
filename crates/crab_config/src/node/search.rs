use crate::error::ConfigError;
use crate::node::{ConfigurationNode, FLAT_PATH_SEPARATOR, NodeContent};

/// Searches for `ConfigurationKey` in a tree given following flat key \
/// Given following yaml:
/// ```yml
/// test:
///     source: 2
/// ```
/// You can access `source` `ConfigurationKey` from tree by using flat key `test.source` \
/// Example:
/// ```rust
/// use crab_config::Value;
/// use crab_config::node::search::get_configuration_node_from_flat_key;
/// use crab_config::node::ConfigurationNode;
///
/// // Implementation of above yaml
/// let configuration_key = ConfigurationNode::with_children(Vec::from_iter([
///     ("test".to_owned(), ConfigurationNode::with_children(Vec::from_iter([
///         ("source".to_owned(), ConfigurationNode::with_value(Value::String("2".to_owned())))
///     ])))
/// ]));
///
/// // Usage of the function
/// let value = get_configuration_node_from_flat_key(&configuration_key, "test.source").unwrap();
/// println!("{:?}", value.get_value().unwrap());
/// ```
pub fn get_configuration_node_from_flat_key(
    configuration_node: &ConfigurationNode,
    key: &str,
) -> Result<ConfigurationNode, ConfigError> {
    let mut split_key: Vec<&str> = key.split(FLAT_PATH_SEPARATOR).collect();
    let first_key = split_key.first().unwrap().to_string();
    if split_key.len() != 1 {
        split_key.remove(0);
    }
    let rest_of_the_key = split_key.join(FLAT_PATH_SEPARATOR);

    let NodeContent::Children(children) = &configuration_node.content else {
        return Err(ConfigError::MissingNode {
            key: rest_of_the_key,
            parent_key: first_key,
        });
    };

    for (local_key, node) in children {
        if local_key == key {
            return Ok(node.clone());
        } else if local_key == &first_key {
            return get_configuration_node_from_flat_key(node, &rest_of_the_key);
        }
    }

    Err(ConfigError::MissingNode {
        key: rest_of_the_key,
        parent_key: first_key,
    })
}
