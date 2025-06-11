use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::player_spawn_information::PlayerSpawnInformation;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "respawn", id = 0x4B, state = Play)]
pub struct RespawnS2CPacket {
    pub spawn_information: PlayerSpawnInformation,
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
