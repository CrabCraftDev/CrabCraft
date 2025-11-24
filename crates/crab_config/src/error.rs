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
    InvalidFieldValue { serde_error: serde_json::Error },
    #[error(
        "Invalid enum config value for enum {} found value: {} Expected values: {:?}",
        enum_name,
        found_value,
        expected_values
    )]
    InvalidEnumConfigValue {
        enum_name: &'static str,
        found_value: String,
        expected_values: &'static str,
    },

    #[error("Config version wasn't found in configuration entry during verification process")]
    ConfigVersionNotFound,
    #[error("Invalid config version")]
    InvalidConfigVersion,

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    SerdeSaphyrSerialization(#[from] serde_saphyr::ser_error::Error),
    #[error(transparent)]
    SerdeSaphyr(#[from] serde_saphyr::Error),
}
