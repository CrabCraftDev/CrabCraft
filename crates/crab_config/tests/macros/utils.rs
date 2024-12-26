use crab_macros::Config;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize, PartialEq, Debug)]
pub struct TestConfig {
    /// This is a test field.
    /// It is used to demonstrate extracting documentation.
    #[default_value(10)]
    pub version: u8,
    #[default_value(150)]
    pub another_one: u8,
    pub test_value: AnotherTestConfig,
}

#[derive(Config, Serialize, Deserialize, PartialEq, Debug)]
pub struct AnotherTestConfig {
    #[default_value(4)]
    pub another_test_value: u8,
}
