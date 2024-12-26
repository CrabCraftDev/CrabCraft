use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use crate::handlers::format_handler::ConfigFormatHandler;
use crate::version_control::modifications::processor::Processor;

use crate::configuration_entry::ConfigurationEntry;
use crate::configuration_entry::configuration_entry::ROOT_ENTRY_NAME;
use crate::errors::Error;
use crate::types::config::Config;

/// Describes name of variable behind saying this is the version of this config
/// Important to use name of it everywhere in every config
const VERSION_CONFIG_FIELD_NAME: &str = "version";

pub struct Configurable<T: Config> {
    config_file_path: String,
    pub implementation: Box<dyn ConfigFormatHandler>,
    /// Placeholder for the generic type `T`.
    /// This ensures the generic type `T` is utilized, as Rust requires all generic types to be used.
    /// Without this, the compiler will throw an error, stating that the type parameter `T` is unused.
    _phantom_data: PhantomData<T>,
}

impl<T: Config> Configurable<T> {
    pub fn new(config_file_path: String, implementation: Box<dyn ConfigFormatHandler>) -> Configurable<T> {
        Configurable::<T> {
            config_file_path,
            implementation,
            _phantom_data: PhantomData,
        }
    }

    pub fn read_and_save_default_if_config_doesnt_exists(&self, processors: Vec<Processor>) -> Result<T, Error> {
        let config_path = self.config_file_path.clone();
        if Path::new(&config_path).exists() {
            return self.read_config_and_verify(processors);
        }

        let default_config = T::default();
        self.write(default_config.as_configuration_entry())?;
        Ok(default_config)
    }

    pub fn read_config_and_verify(&self, processors: Vec<Processor>) -> Result<T, Error> {
        let mut entry = self.read_config()?;
        self.verify_config(&mut entry, processors)
    }

    fn verify_config(
        &self,
        configuration_entry: &mut ConfigurationEntry,
        processors: Vec<Processor>,
    ) -> Result<T, Error> {
        // !IMPORTANT
        // There's no physical way to know where the config_version is so we just assume it's
        // going to be in root node if it ain't there we just panic
        let maybe_config_version = configuration_entry.get_value_from_flat_path(VERSION_CONFIG_FIELD_NAME);
        if maybe_config_version.is_none() {
            return Err(Error::ConfigVersionNotFound);
        }

        let config_version = maybe_config_version
            .unwrap()
            .as_i64()
            .ok_or(Error::InvalidConfigVersion)?;

        let modified_result =
            Self::apply_changelogs_to_configuration_entry(configuration_entry, config_version as u32, processors);

        modified_result.map(|modified_configuration_entry| T::map(modified_configuration_entry.clone()))
    }

    /// Despite having almost same name as `read_config_and_verify` it returns `ConfigurationEntry`
    /// due to not having certainty of type this function doesn't check for type
    /// just dumps config into more manageable format of `ConfigurationEntry`
    pub fn read_config(&self) -> Result<ConfigurationEntry, Error> {
        let config_file_path = &self.config_file_path;
        let mut configuration_entry = self.implementation.read(String::from(config_file_path))?;

        // TODO: In future find/create library that handles reading documentation comments above entry :)
        // Current implementation just copies over the documentation from already written docs from code
        configuration_entry.copy_docs_from(&ConfigurationEntry::new(
            "root".to_string(),
            Some(T::default().as_configuration_entry()),
            None,
            true,
        ));

        Ok(configuration_entry)
    }

    pub fn write(&self, config_entry: Vec<ConfigurationEntry>) -> Result<(), Error> {
        self.write_node(ConfigurationEntry::new(
            ROOT_ENTRY_NAME.to_string(),
            Some(config_entry),
            None,
            false,
        ))
    }

    pub fn write_node(&self, node: ConfigurationEntry) -> Result<(), Error> {
        self.implementation.write(self.config_file_path.clone(), node)
    }

    fn apply_changelogs_to_configuration_entry(
        mut configuration_entry: &mut ConfigurationEntry,
        config_version: u32,
        processors: Vec<Processor>,
    ) -> Result<&ConfigurationEntry, Error> {
        let mut changelog_version_to_processor: HashMap<u32, Processor> = HashMap::new();

        for processor in processors {
            if processor.get_config_applied_version() < config_version {
                continue;
            }
            changelog_version_to_processor.insert(processor.get_config_applied_version(), processor);
        }
        let highest_config_version = changelog_version_to_processor
            .iter()
            .max_by(|a, b| a.0.cmp(b.0))
            .map(|(k, _v)| k)
            .unwrap_or_else(|| &0u32);

        for version in config_version..=*highest_config_version {
            if let Some(processor) = changelog_version_to_processor.get(&version) {
                configuration_entry = (processor.processor_function)(configuration_entry);
            }
        }

        Ok(configuration_entry)
    }
}
