use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::player_spawn_information::PlayerSpawnInformation;
use crate::types::varint::VarInt;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "login", id = 0x2C, state = Play)]
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
    pub spawn_information: PlayerSpawnInformation,
    pub enforces_secure_chat: bool,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct LastDeathLocation {
    pub dimension_id: NamespacedKey,
    pub location: BlockPosition,
}
