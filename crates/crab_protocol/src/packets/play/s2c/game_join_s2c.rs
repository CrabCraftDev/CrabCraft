use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::game_mode::{GameMode, OptionalGameMode};
use crate::types::namespaced_key::NamespacedKey;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2B, state = Play)]
pub struct GameJoinS2CPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<NamespacedKey>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub max_players: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub view_distance: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    /// The ID of the type of dimension in the minecraft:dimension_type registry, defined by the Registry Data packet.
    #[protocol_attribute(protocol_type = VarInt)]
    pub dimension_type: i32,
    pub dimension_name: NamespacedKey,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: OptionalGameMode,
    pub is_debug_world: bool,
    pub is_flat_world: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
    pub enforces_secure_chat: bool,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct LastDeathLocation {
    pub dimension_id: NamespacedKey,
    pub location: BlockPosition,
}
