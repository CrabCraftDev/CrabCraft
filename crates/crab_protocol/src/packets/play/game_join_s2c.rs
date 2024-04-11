use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;
use crate::types::game_mode::{GameMode, OptionalGameMode};

#[derive(Packet, Debug, Clone)]
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
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct LastDeathLocation {
    pub dimension_id: Identifier,
    pub location: BlockPosition,
}
