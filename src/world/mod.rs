use crab_text::utils::namespaced_key::NamespacedKey;
use std::sync::LazyLock;

pub static OVERWORLD_IDENTIFIER: LazyLock<NamespacedKey> =
    LazyLock::new(|| NamespacedKey::minecraft("overworld").unwrap());

pub use world_data::*;

pub mod chunk_sender;
mod chunk_view;
mod world_data;
pub mod worlds_storage;
