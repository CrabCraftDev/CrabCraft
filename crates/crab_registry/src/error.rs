use crab_macros::DetailedFrom;
use crab_text::error::{DetailedTextError, TextError};
use crab_text::utils::namespaced_key::NamespacedKey;
use thiserror::Error;

#[derive(Debug, DetailedFrom, Error)]
pub enum RegistryError {
    #[error("There is no key '{key}' in registry {registry}")]
    MissingKeyInRegistry { key: NamespacedKey, registry: &'static str },
    #[error("There is no entry with numeric id {id} in registry {registry}")]
    MissingIdInRegistry { id: usize, registry: &'static str },
    #[error("There is no vanilla registry '{registry}' in assets/datapack/")]
    MissingVanillaRegistry { registry: &'static str },

    #[error("Registry '{registry}' was not found in the root registry")]
    RegistryNotFound { registry: &'static str },
    #[error("Registry type mismatch for '{registry}'. Make sure you request the correct typed registry")]
    RegistryTypeMismatch { registry: &'static str },

    #[error("Failed to parse JSON for '{path}': {source}")]
    JsonParseError { path: String, source: serde_json::Error },
    #[error(transparent)]
    Text(
        #[from]
        #[detailed(DetailedTextError)]
        TextError,
    ),
}
