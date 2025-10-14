use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Global registry has not been initialized")]
    RegistryNotInitialized,
}
