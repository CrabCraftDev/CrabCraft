use bitfield_struct::bitfield;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x38, state = Play)]
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
    pub allow_flying: bool,
    pub creative_mode: bool,
    #[bits(4)]
    __: u8,
}
