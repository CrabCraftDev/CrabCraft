use bitfield_struct::bitfield;
use crab_macros::ProtocolSerializer;

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct PlayerMovementFlags {
    pub on_ground: bool,
    pub horizontal_collision: bool,
    #[bits(6)]
    __: u8,
}
