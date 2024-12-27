use crate::error::ConfigError;
use crate::node::ConfigurationNode;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_yml::{from_value, to_value};

/// Used to implement serializing specific structs inside config
pub trait Config: Default {
    fn from_configuration_node(node: &ConfigurationNode) -> Result<Self, ConfigError>;

    fn as_configuration_node(&self) -> ConfigurationNode;
}

// Default config trait implementation for primitive types
// Improve when https://github.com/rust-lang/rust/issues/31844 is fully stabilized

pub trait PrimitiveConfig: Default + Serialize + DeserializeOwned {}
impl PrimitiveConfig for i8 {}
impl PrimitiveConfig for i32 {}
impl PrimitiveConfig for i64 {}
impl PrimitiveConfig for u8 {}
impl PrimitiveConfig for u32 {}
impl PrimitiveConfig for u64 {}
impl PrimitiveConfig for bool {}
impl PrimitiveConfig for f32 {}
impl PrimitiveConfig for f64 {}
impl PrimitiveConfig for String {}
impl<T: PrimitiveConfig> PrimitiveConfig for Vec<T> {}

impl<T> Config for T
where
    T: PrimitiveConfig,
{
    fn from_configuration_node(node: &ConfigurationNode) -> Result<Self, ConfigError> {
        let value = node.get_value().ok_or(ConfigError::FoundValueExpectedMapping)?;

        from_value(value.clone()).map_err(|error| ConfigError::InvalidFieldValue { serde_error: error })
    }

    fn as_configuration_node(&self) -> ConfigurationNode {
        ConfigurationNode::with_value(to_value(self).unwrap())
    }
}
