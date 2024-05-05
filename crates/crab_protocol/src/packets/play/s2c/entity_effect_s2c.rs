use bitfield_struct::bitfield;
use crab_nbt::Nbt;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x72, state = Play)]
pub struct EntityEffectS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub effect_id: i32,
    pub amplifier: i8,
    #[protocol_attribute(protocol_type = VarInt)]
    pub duration: i32,
    pub has_factor_data: bool,
    pub factor_data: Nbt,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct EntityEffectFlags {
    pub is_ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    #[bits(5)]
    __: u8,
}
