use crab_text::utils::namespaced_key::NamespacedKey;
use std::sync::LazyLock;

pub static OVERWORLD_IDENTIFIER: LazyLock<NamespacedKey> =
    LazyLock::new(|| NamespacedKey::minecraft("overworld").unwrap());

pub mod storage;
