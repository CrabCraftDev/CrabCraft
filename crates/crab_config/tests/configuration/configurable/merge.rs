use crate::utils::get_test_configuration_node;
use crab_config::node::ConfigurationNode;
use serde_json::Value;

#[test]
fn merge_test() {
    let mut default_config = get_test_configuration_node();

    let user_config = ConfigurationNode::with_children(Vec::from_iter([
        (
            "1".to_string(),
            ConfigurationNode::with_value(Value::String("Test2".to_string())),
        ),
        (
            "old".to_string(),
            ConfigurationNode::with_value(Value::String("Test2".to_string())),
        ),
        (
            "123".to_string(),
            ConfigurationNode::with_value(Value::String("old value".to_string())),
        ),
        (
            "1234".to_string(),
            ConfigurationNode::with_children(Vec::from_iter([(
                "12345".to_string(),
                ConfigurationNode::with_value(Value::String("Test".to_string())),
            )])),
        ),
    ]));

    default_config.merge(&user_config);

    let final_config = ConfigurationNode::with_children(Vec::from_iter([
        (
            "1".to_string(),
            ConfigurationNode::with_value(Value::String("Test2".to_string())),
        ),
        (
            "12".to_string(),
            ConfigurationNode::with_value(Value::String("Test".to_string())),
        ),
        (
            "123".to_string(),
            ConfigurationNode::with_value(Value::String("old value".to_string())),
        ),
        (
            "1234".to_string(),
            ConfigurationNode::with_children(Vec::from_iter([(
                "12345".to_string(),
                ConfigurationNode::with_value(Value::String("Test".to_string())),
            )])),
        ),
    ]));

    assert_eq!(default_config, final_config);
}
