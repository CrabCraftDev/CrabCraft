use crate::macros::utils::{AnotherTestConfig, TestConfig};
use crab_config::types::config::Config;
use std::collections::HashMap;

#[test]
fn struct_to_configuration_entry() {
    const EXPECTED_VERSION_DOC: &str = " This is a test field.\n It is used to demonstrate extracting documentation.";

    let x = TestConfig {
        version: 2,
        another_one: 1,
        test_value: AnotherTestConfig { another_test_value: 1 },
    };
    let configuration_entries = x.as_configuration_entry();
    let entries: Vec<_> = configuration_entries.into_iter().collect();

    assert_eq!(
        entries.len(),
        3,
        "Expected entries for version, another_one, and test_value"
    );

    let expected_docs = [
        ("version", Some(EXPECTED_VERSION_DOC)),
        ("another_one", None), // Assuming this field has no documentation
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    for entry in entries {
        let expected_doc = expected_docs.get(entry.key.as_str()).unwrap_or(&None);
        assert_eq!(entry.documentation.as_deref(), *expected_doc);
    }
}
