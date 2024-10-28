use crate::configuration_entry::ConfigurationEntry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

/// Used to implement serializing specific structs inside config
pub trait Config: Default {
    fn map(config_entry: ConfigurationEntry) -> Self;

    fn map_to_configuration_entry(&self) -> Vec<ConfigurationEntry>;

    /// Gives out field_name -> default value intended to be stored
    /// for example with struct:
    /// ```rust
    ///
    /// use crab_macros::Config;
    /// #[derive(Config)]
    /// struct MyConfig {
    ///     #[default_value("200")]
    ///     some_field: String
    /// }
    /// ```
    /// Will give following hashmap:
    /// some_field -> 200
    fn get_default_fields(&self) -> HashMap<String, String>;
}

// Implement Config for common types like String, u16 etc.
impl<T: FromStr + Default> Config for T
where
    T::Err: Debug,
{
    fn map(config_entry: ConfigurationEntry) -> Self {
        config_entry.get_value().unwrap().parse::<T>().unwrap()
    }

    fn map_to_configuration_entry(&self) -> Vec<ConfigurationEntry> {
        unimplemented!("Config's map_to_configuration_entry is not implemented!")
    }

    fn get_default_fields(&self) -> HashMap<String, String> {
        unimplemented!("Config's get_default_fields is not implemented!")
    }
}
