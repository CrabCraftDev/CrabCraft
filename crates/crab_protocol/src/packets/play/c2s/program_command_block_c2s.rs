use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_command_block", id = 0x35, state = Play)]
pub struct ProgramCommandBlockC2SPacket {
    pub position: BlockPosition,
    pub command: String,
    pub command_block_mode: CommandBlockMode,
    pub command_block_flags: CommandBlockFlags,
}

#[derive(ProtocolSerializer, FromRepr, Copy, Clone, Debug)]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct CommandBlockFlags {
    pub track_output: bool,
    pub is_conditional: bool,
    pub automatic: bool,
    #[bits(5)]
    __: u8,
}
