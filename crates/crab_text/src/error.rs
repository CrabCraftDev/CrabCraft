use crab_nbt::error::Error as NbtError;
use serde_json::error::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextError {
    #[error("Invalid namespaced key {0}")]
    InvalidNamespacedKeyFormat(String),
    #[error("JsonTextComponent too long")]
    JsonTextComponentTooLong,
    #[error("Selected format: {format} doesn't support {message}")]
    UnsupportedFormatOperation { format: String, message: String },

    #[error("Legacy text format does not support serializing RGB colors")]
    LegacyFormatDoesNotSupportRgb,
    #[error("Legacy text format supports serialization only for literal text")]
    LegacyFormatSupportsOnlyLiteral,

    #[error(transparent)]
    Nbt(#[from] NbtError),
    #[error(transparent)]
    Json(#[from] JsonError),
}
