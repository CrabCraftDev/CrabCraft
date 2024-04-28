use bitfield_struct::bitfield;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x23, state = Play)]
pub struct PlayerInputC2SPacket {
    /// Positive to the left of the player.
    pub sideways_motion: f32,
    pub forward_motion: f32,
    pub flags: InputFlags,
}

#[bitfield(u8)]
pub struct InputFlags {
    pub jump: bool,
    pub unmount: bool,
    #[bits(6)]
    __: u8,
}

impl ProtocolType for InputFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self(buf.get_i8() as u8))
    }
}
