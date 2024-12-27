use crate::macros::utils::{AnotherTestConfig, TestConfig};
use crab_config::configurable::VERSION_CONFIG_FIELD_NAME;
use crab_config::types::config::Config;
use std::collections::HashMap;

#[test]
fn struct_to_configuration_node() {
    const EXPECTED_VERSION_DOC: &str =
        " This is a test field.\n It is used to demonstrate extracting documentation.\n Default value: 10";

    let x = TestConfig {
        _version: 2,
        another_one: 1,
        test_value: AnotherTestConfig { another_test_value: 1 },
    };
    let entries = x.as_configuration_node();
    let entries = entries.get_children().unwrap();

    assert_eq!(
        entries.len(),
        3,
        "Expected entries for version, another_one, and test_value"
    );

    let expected_docs = [
        (VERSION_CONFIG_FIELD_NAME, Some(EXPECTED_VERSION_DOC)),
        ("another_one", Some(" Default value: 150")), // Assuming this field has no documentation
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    for (key, node) in entries {
        let expected_doc = expected_docs.get(key.as_str()).unwrap_or(&None);
        assert_eq!(
            node.documentation.as_deref(),
            *expected_doc,
            "Invalid documentation for entry: {}",
            key
        );
    }
}
