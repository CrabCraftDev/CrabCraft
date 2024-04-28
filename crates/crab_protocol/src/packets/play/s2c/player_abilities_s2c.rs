use bitfield_struct::bitfield;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x36, state = Play)]
pub struct PlayerAbilitiesS2CPacket {
    pub flags: PlayerAbilitiesFlags,
    pub flying_speed: f32,
    pub field_of_view_modifier: f32,
}

#[bitfield(u8)]
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

impl ProtocolType for PlayerAbilitiesFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(PlayerAbilitiesFlags(buf.get_i8() as u8))
    }
}
