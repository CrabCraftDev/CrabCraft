use crate::key::RegistryKey;
use crate::registry::TypedRegistryBase;
use std::fmt::Debug;

/// Marker trait for types that live inside a registry.
///
/// Each `Registrable` type must point to the registry it belongs to via the
/// `REGISTRY` associated constant. This lets the `RootRegistry` look up the
/// correct `Registry<T>` without passing types around at call sites.
pub trait Registrable: Send + Sync + Debug + Sized + 'static {
    // FIXME: Feature `associated type defaults` is not available - add here SimpleRegistry<Self>
    type RegistryType: TypedRegistryBase<Value = Self> + 'static;

    const REGISTRY: RegistryKey<Self>;
}
