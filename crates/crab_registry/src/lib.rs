#![doc = include_str!("../README.md")]

use arc_swap::ArcSwap;
use linkme::distributed_slice;
use std::sync::LazyLock;

pub use registrable::*;
pub use registry::*;
pub use root::RootRegistry;

pub mod error;
pub mod internal;
pub mod key;
mod registrable;
mod registry;
mod root;
pub mod vanilla;
mod vanilla_populator;

#[distributed_slice]
pub static REGISTRY_POPULATORS: [fn(&mut RootRegistry)];

pub static REGISTRY: LazyLock<ArcSwap<RootRegistry>> =
    LazyLock::new(|| ArcSwap::from_pointee(RootRegistry::populated()));
