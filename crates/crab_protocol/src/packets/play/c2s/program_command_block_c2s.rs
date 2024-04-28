use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2D, state = Play)]
pub struct ProgramCommandBlockC2SPacket {
    pub position: BlockPosition,
    pub command: String,
    pub command_block_mode: CommandBlockMode,
    pub command_block_flags: CommandBlockFlags,
}

#[derive(ProtocolType, FromRepr, Copy, Clone, Debug)]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}

#[bitfield(u8)]
pub struct CommandBlockFlags {
    pub track_output: bool,
    pub is_conditional: bool,
    pub automatic: bool,
    #[bits(5)]
    __: u8,
}

impl ProtocolType for CommandBlockFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u8(self.0);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(CommandBlockFlags(buf.get_u8()))
    }
}
