use bitfield_struct::bitfield;
use crab_macros::ProtocolSerializer;
use evenio::component::Component;

#[bitfield(u8)]
#[derive(ProtocolSerializer, Component)]
pub struct PlayerMovementFlags {
    pub on_ground: bool,
    pub horizontal_collision: bool,
    #[bits(6)]
    __: u8,
}
