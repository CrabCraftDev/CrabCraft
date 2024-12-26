use crate::configuration_entry::ConfigurationEntry;
use serde_yml::{from_value, Value};
use std::collections::HashMap;
use std::str::FromStr;

/// Used to implement serializing specific structs inside config
pub trait Config: Default {
    fn map(config_entry: ConfigurationEntry) -> Self;

    fn as_configuration_entry(&self) -> Vec<ConfigurationEntry>;

    /// Gives out field_name -> default value intended to be stored
    /// for example with struct:
    /// ```rust
    ///
    /// use crab_macros::Config;
    /// #[derive(Config)]
    /// struct MyConfig {
    ///     #[default_value(200)]
    ///     some_field: String
    /// }
    /// ```
    /// Will give following hashmap:
    /// some_field -> 200
    fn get_default_fields(&self) -> HashMap<String, Value>;
}

impl<T: serde::de::DeserializeOwned + Default + FromStr> Config for T {
    fn map(config_entry: ConfigurationEntry) -> Self {
        from_value(config_entry.get_value().unwrap().clone()).unwrap()
    }

    fn as_configuration_entry(&self) -> Vec<ConfigurationEntry> {
        unimplemented!("Config's map_to_configuration_entry is not implemented!")
    }

    fn get_default_fields(&self) -> HashMap<String, Value> {
        unimplemented!("Config's get_default_fields is not implemented!")
    }
}
