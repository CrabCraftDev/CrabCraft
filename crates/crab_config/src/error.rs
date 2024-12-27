use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file {configuration_file_path} wasn't found")]
    ConfigurationFileNotFound { configuration_file_path: String },
    #[error("Invalid configuration file syntax")]
    InvalidConfigSyntax,

    /// Combined error that stores all the errors found during validation
    #[error("Found {} errors in configuration file. Errors: {:?}", errors.len(), errors)]
    InvalidConfig { errors: Vec<ConfigError> },
    #[error("Could not find node with key {key} in parent node with key {parent_key}")]
    MissingNode { key: String, parent_key: String },
    #[error("Found configuration node with value, expected mapping")]
    FoundValueExpectedMapping,
    #[error("Invalid field value ({})", serde_error.to_string())]
    InvalidFieldValue { serde_error: serde_yml::Error },

    #[error("Config version wasn't found in configuration entry during verification process")]
    ConfigVersionNotFound,
    #[error("Invalid config version")]
    InvalidConfigVersion,

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_yml::modules::error::Error),
}
