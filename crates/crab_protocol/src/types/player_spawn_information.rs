use crab_macros::ProtocolSerializer;

use crate::packets::play::LastDeathLocation;
use crate::types::game_mode::GameMode;
use crate::types::game_mode::OptionalGameMode;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PlayerSpawnInformation {
    /// The ID of the type of dimension in the minecraft:dimension_type registry, defined by the Registry Data packet.
    #[protocol_attribute(protocol_type = VarInt)]
    pub dimension_type: i32,
    pub dimension_name: NamespacedKey,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    #[protocol_attribute(protocol_type = OptionalGameMode)]
    pub previous_game_mode: Option<GameMode>,
    pub is_debug_world: bool,
    pub is_flat_world: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub sea_level: i32,
}
