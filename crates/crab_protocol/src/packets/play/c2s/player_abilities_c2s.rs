use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_abilities", id = 0x25, state = Play)]
pub struct PlayerAbilitiesC2SPacket {
    pub flags: ClientPlayerAbilitiesFlags,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct ClientPlayerAbilitiesFlags {
    // Note: First bit is unused. Don't remove this field.
    #[bits(1)]
    __: u8,
    pub is_flying: bool,
    #[bits(6)]
    __: u8,
}
