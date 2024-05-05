use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::game_mode::{GameMode, OptionalGameMode};
use crate::types::identifier::Identifier;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x29, state = Play)]
pub struct GameJoinS2CPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub max_players: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub view_distance: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: Identifier,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: OptionalGameMode,
    pub is_debug_world: bool,
    pub is_flat_world: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct LastDeathLocation {
    pub dimension_id: Identifier,
    pub location: BlockPosition,
}
