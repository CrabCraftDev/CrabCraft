use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Next value in tree is empty")]
    NextValueIsEmpty,
    #[error("Value is empty")]
    ValueIsEmpty,
    #[error("Configuration file {configuration_file_path} wasn't found")]
    ConfigurationFileNotFound { configuration_file_path: String },
    #[error("Undefined behavior")]
    UndefinedBehavior,
    #[error("Matched unsupported type of sequence is_sequence: {is_sequence} is_mapping: {is_mapping}")]
    MatchedUnsupportedTypes { is_sequence: bool, is_mapping: bool },
    #[error("No entries below root")]
    NoEntriesBelowRoot,
    #[error("Element not found")]
    ElementNotFound,
    #[error("Key cannot be empty")]
    EmptyKey,
    #[error("Root entry can't have any value")]
    RootCanNotHaveValue,
    #[error("Root entry can't have value and children at the same time")]
    NodeCanNotHaveValue,

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_yml::modules::error::Error),
    #[error("Unknown Error")]
    Unknown,

    #[error("Config version wasn't found in configuration entry during verification process")]
    ConfigVersionNotFound,
    #[error("Invalid config version")]
    InvalidConfigVersion,
}
