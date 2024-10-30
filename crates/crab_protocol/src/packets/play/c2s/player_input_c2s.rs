use bitfield_struct::bitfield;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x28, state = Play)]
pub struct PlayerInputC2SPacket {
    pub movement: InputFlags,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct InputFlags {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shift: bool,
    pub sprint: bool,
    #[bits(1)]
    __: u8,
}
