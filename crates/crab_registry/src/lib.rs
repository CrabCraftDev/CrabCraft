pub use registry::RootRegistry;
use std::sync::LazyLock;

use arc_swap::ArcSwap;
use linkme::distributed_slice;

pub mod error;
pub mod loader;
pub mod registry;
pub mod registry_key;
pub mod vanilla;

#[distributed_slice]
pub static REGISTRY_POPULATORS: [fn(&mut RootRegistry)];

pub static REGISTRY: LazyLock<ArcSwap<RootRegistry>> = LazyLock::new(|| ArcSwap::from_pointee(RootRegistry::new()));
