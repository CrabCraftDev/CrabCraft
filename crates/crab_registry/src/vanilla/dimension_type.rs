use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DimensionType {
    // FIXME: Use those fields to remove any arbitrary constant dimension values from our codebase
    pub height: i32,
    /// The maximum height to which chorus fruits and nether portals can bring players within this dimension.
    /// Must be lower than `height`.
    pub logical_height: i32,
    pub min_y: i32,
}

impl Registrable for DimensionType {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:dimension_type");
}
