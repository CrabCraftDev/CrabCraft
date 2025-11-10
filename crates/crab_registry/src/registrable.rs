use std::any::Any;

use crate::key::RegistryKey;

/// Marker trait for types that live inside a registry.
///
/// Each `Registrable` type must point to the registry it belongs to via the
/// `REGISTRY` associated constant. This lets the `RootRegistry` look up the
/// correct `Registry<T>` without passing types around at call sites.
pub trait Registrable: Sized + Send + Sync + Any + 'static {
    const REGISTRY: RegistryKey<Self>;
}
