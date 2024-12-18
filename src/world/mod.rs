use crab_protocol::types::namespaced_key::NamespacedKey;
use std::sync::LazyLock;

pub static OVERWORLD_IDENTIFIER: LazyLock<NamespacedKey> =
    LazyLock::new(|| NamespacedKey::minecraft("overworld").unwrap());

pub use world_data::*;

mod world_data;
pub mod worlds_storage;
