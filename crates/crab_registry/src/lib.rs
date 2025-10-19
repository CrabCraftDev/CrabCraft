#![doc = include_str!("../README.md")]

use crate::{error::RegistryError, registry::RootRegistry};
use arc_swap::ArcSwap;
use linkme::distributed_slice;
pub use registrable::*;
use std::sync::LazyLock;

pub mod crabcraft;
pub mod error;
pub mod key;
pub mod minecraft;
mod registrable;
pub mod registry;

#[distributed_slice]
pub static REGISTRY_POPULATORS: [fn(&mut RootRegistry) -> Result<(), RegistryError>];

pub static REGISTRY: LazyLock<ArcSwap<RootRegistry>> =
    LazyLock::new(|| ArcSwap::from_pointee(RootRegistry::populated().expect("Error populating root registry")));
