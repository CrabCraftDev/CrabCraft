use bitfield_struct::bitfield;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x20, state = Play)]
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
