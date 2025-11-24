use crab_config::node::{ConfigurationNode, NodeContent};
use serde_json::Value;

mod entry_modify;
mod flat_path_removal;
mod path_addition;

#[test]
fn new_node_with_value() {
    let value = Value::String("test_value".to_string());
    let node = ConfigurationNode::with_value(value.clone());
    assert_eq!(node.get_value(), Some(&value));
    assert!(node.get_children().is_none());
}

#[test]
fn new_node_with_children() {
    let children = vec![("child".to_string(), ConfigurationNode::default())];
    let node = ConfigurationNode::with_children(children.clone());
    assert!(node.get_children().is_some());
    assert!(node.get_value().is_none());
}

#[test]
fn set_value_by_flat_path() {
    let mut node = ConfigurationNode::default();
    let value = Value::String("test_value".to_string());
    assert!(node.set_value_by_flat_path("a.b.c", value.clone()));
    assert_eq!(node.get_value_from_flat_path("a.b.c"), Some(&value));
}

#[test]
fn get_value_from_flat_path_nonexistent() {
    let node = ConfigurationNode::default();
    assert!(node.get_value_from_flat_path("non.existent.path").is_none());
}

#[test]
fn merge_nodes() {
    let mut node1 = ConfigurationNode::default();
    let mut node2 = ConfigurationNode::default();
    let value = Value::String("test_value".to_string());
    node2.content = NodeContent::Value(value.clone());
    node1.merge(&node2);
    assert_eq!(node1.get_value(), Some(&value));
}

#[test]
#[should_panic]
fn execute_for_configuration_node_in_flat_path() {
    let mut node = ConfigurationNode::default();
    let value = Value::String("test_value".to_string());
    node.set_value_by_flat_path("a.b.c", value.clone());
    node.execute_for_configuration_node_in_flat_path("a.b.c", &|_| panic!("It should fail"));
}
