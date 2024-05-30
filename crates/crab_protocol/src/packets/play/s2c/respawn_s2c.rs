use bitfield_struct::bitfield;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::packets::play::LastDeathLocation;
use crate::protocol::ProtocolState::*;
use crate::types::game_mode::GameMode;
use crate::types::game_mode::OptionalGameMode;
use crate::types::identifier::Identifier;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x47, state = Play)]
pub struct RespawnS2CPacket {
    /// The ID of the type of dimension in the minecraft:dimension_type registry, defined by the Registry Data packet.
    #[protocol_attribute(protocol_type = VarInt)]
    pub dimension_type: i32,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    #[protocol_attribute(protocol_type = OptionalGameMode)]
    pub previous_game_mode: Option<GameMode>,
    pub is_debug_world: bool,
    pub is_flat_world: bool,
    pub last_death_location: Option<LastDeathLocation>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub portal_cooldown: i32,
    pub data_kept: DataKeptAfterRespawn,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct DataKeptAfterRespawn {
    pub keep_attributes: bool,
    pub keep_meta: bool,
    #[bits(6)]
    __: u8,
}
