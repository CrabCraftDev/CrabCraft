use bitfield_struct::bitfield;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x23, state = Play)]
pub struct PlayerInputC2SPacket {
    /// Positive to the left of the player.
    pub sideways_motion: f32,
    pub forward_motion: f32,
    pub flags: InputFlags,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct InputFlags {
    pub jump: bool,
    pub unmount: bool,
    #[bits(6)]
    __: u8,
}
