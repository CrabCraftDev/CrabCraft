use crate::error::ConfigError;
use crate::handlers::format_handler::ConfigFormatHandler;
use crate::node::ConfigurationNode;
use crate::types::config::Config;
use crate::version_control::migrations::migration::Migration;
use semver::Version;
use serde_json::Value;
use std::marker::PhantomData;
use std::path::Path;

/// Describes name of variable behind saying this is the version of this config
pub const VERSION_CONFIG_FIELD_NAME: &str = "_version";

pub trait ConfigVersion: PartialOrd + Ord + PartialEq {
    fn from_value(value: &Value) -> Result<Self, ConfigError>
    where
        Self: Sized;

    fn to_value(&self) -> Value;
}

impl ConfigVersion for Version {
    fn from_value(value: &Value) -> Result<Self, ConfigError> {
        let value = value.as_str().ok_or(ConfigError::InvalidConfigVersion)?;
        Version::parse(value).map_err(|_| ConfigError::InvalidConfigVersion)
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

pub struct Configurable<T: Config, V: ConfigVersion> {
    config_file_path: String,
    config_version: Option<V>,
    pub implementation: Box<dyn ConfigFormatHandler>,
    /// Placeholder for the generic type `T`.
    /// This ensures the generic type `T` is utilized, as Rust requires all generic types to be used.
    /// Without this, the compiler will throw an error, stating that the type parameter `T` is unused.
    _phantom_data: PhantomData<T>,
}

impl<T: Config, V: ConfigVersion> Configurable<T, V> {
    pub fn new(config_file_path: String, implementation: Box<dyn ConfigFormatHandler>) -> Configurable<T, V> {
        Configurable::<T, V> {
            config_file_path,
            implementation,
            config_version: None,
            _phantom_data: PhantomData,
        }
    }

    pub fn new_with_version(
        config_file_path: String,
        implementation: Box<dyn ConfigFormatHandler>,
        version: Option<V>,
    ) -> Configurable<T, V> {
        Configurable::<T, V> {
            config_file_path,
            implementation,
            config_version: version,
            _phantom_data: PhantomData,
        }
    }

    pub fn load_or_create_default_config(&self, migrations: Vec<Box<dyn Migration<V>>>) -> Result<T, ConfigError> {
        let config_path = self.config_file_path.clone();
        if Path::new(&config_path).exists() {
            return self.read_config_and_update(migrations);
        }

        let default_config = T::default();
        self.write_node(default_config.as_configuration_node())?;
        Ok(default_config)
    }

    pub fn read_config_and_update(&self, migrations: Vec<Box<dyn Migration<V>>>) -> Result<T, ConfigError> {
        let mut entry = self.read_config()?;
        self.apply_migrations(&mut entry, migrations)
    }

    fn apply_migrations(
        &self,
        configuration_node: &mut ConfigurationNode,
        migrations: Vec<Box<dyn Migration<V>>>,
    ) -> Result<T, ConfigError> {
        let config_version = if let Some(config_version) = &self.config_version {
            config_version
        } else {
            let Some(config_version) = configuration_node.get_value_from_flat_path(VERSION_CONFIG_FIELD_NAME) else {
                return Err(ConfigError::ConfigVersionNotFound);
            };

            &V::from_value(config_version)?
        };

        let mut default_configuration_node = T::default().as_configuration_node();

        let default_version = V::from_value(
            default_configuration_node
                .get_value_from_flat_path(VERSION_CONFIG_FIELD_NAME)
                .unwrap(),
        )?;

        if config_version != &default_version {
            Self::apply_migrations_to_node(configuration_node, config_version, &default_version, migrations)?;

            default_configuration_node.merge(configuration_node);
            default_configuration_node.set_value_by_flat_path(VERSION_CONFIG_FIELD_NAME, default_version.to_value());
            self.write_node(default_configuration_node.clone())?;

            T::from_configuration_node(&default_configuration_node)
        } else {
            T::from_configuration_node(configuration_node)
        }
    }

    /// Despite having almost same name as `read_config_and_verify` it returns `ConfigurationNode`
    /// due to not having certainty of type this function doesn't check for type
    /// just dumps config into more manageable format of `ConfigurationNode`
    pub fn read_config(&self) -> Result<ConfigurationNode, ConfigError> {
        let config_file_path = self.config_file_path.clone();
        let mut configuration_node = self.implementation.read(config_file_path)?;

        configuration_node.copy_docs_from(&T::default().as_configuration_node());

        Ok(configuration_node)
    }

    pub fn write_node(&self, node: ConfigurationNode) -> Result<(), ConfigError> {
        self.implementation.write(self.config_file_path.clone(), node)
    }

    fn apply_migrations_to_node(
        configuration_node: &mut ConfigurationNode,
        previous_config_version: &V,
        config_version: &V,
        mut migrations: Vec<Box<dyn Migration<V>>>,
    ) -> Result<(), ConfigError> {
        migrations.sort_by_key(|a| a.version());

        for migration in migrations {
            if &migration.version() > previous_config_version && &migration.version() <= config_version {
                migration.up(configuration_node);
            }
        }

        Ok(())
    }
}
