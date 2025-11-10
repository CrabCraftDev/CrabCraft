use crate::key::RegistryKey;
use crate::registrable::Registrable;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WolfSoundVariant {}

impl Registrable for WolfSoundVariant {
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:wolf_sound_variant");
}
