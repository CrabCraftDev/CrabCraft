use bitfield_struct::bitfield;
use crab_nbt::Nbt;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
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
pub struct EntityEffectFlags {
    pub is_ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    #[bits(5)]
    __: u8,
}

impl ProtocolType for EntityEffectFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(EntityEffectFlags(buf.get_i8() as u8))
    }
}
