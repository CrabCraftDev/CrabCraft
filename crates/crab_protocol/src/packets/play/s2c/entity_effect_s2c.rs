use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x7D, state = Play)]
pub struct EntityEffectS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub effect_id: i32,
    pub amplifier: i8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub duration: i32,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct EntityEffectFlags {
    pub is_ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    /// Currently only used in the darkness effect
    pub blend: bool,
    #[bits(4)]
    __: u8,
}
