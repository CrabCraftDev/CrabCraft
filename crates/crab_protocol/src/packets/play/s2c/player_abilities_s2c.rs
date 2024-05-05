use bitfield_struct::bitfield;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x36, state = Play)]
pub struct PlayerAbilitiesS2CPacket {
    pub flags: PlayerAbilitiesFlags,
    pub flying_speed: f32,
    pub field_of_view_modifier: f32,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct PlayerAbilitiesFlags {
    pub invulnerable: bool,
    pub flying: bool,
    #[bits(1)]
    __: u8,
    pub allow_flying: bool,
    #[bits(3)]
    __: u8,
    pub creative_mode: bool,
}
