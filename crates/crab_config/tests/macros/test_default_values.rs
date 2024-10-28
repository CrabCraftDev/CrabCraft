use crate::macros::utils::{AnotherTestConfig, TestConfig};

#[test]
fn test_default_values() {
    let expected_object = TestConfig {
        version: 10,
        another_one: 150,
        test_value: AnotherTestConfig { another_test_value: 4 },
    };

    assert_eq!(TestConfig::default(), expected_object)
}
