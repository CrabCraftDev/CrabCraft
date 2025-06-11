use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::varlong::VarLong;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_structure_block", id = 0x39, state = Play)]
pub struct ProgramStructureBlockC2SPacket {
    pub position: BlockPosition,
    pub structure_block_action: StructureBlockAction,
    pub structure_block_mode: StructureBlockMode,
    pub name: String,
    pub offset_x: i8,
    pub offset_y: i8,
    pub offset_z: i8,
    pub size_x: i8,
    pub size_y: i8,
    pub size_z: i8,
    pub mirror_mode: MirrorMode,
    pub rotation_mode: RotationMode,
    #[protocol_attribute(bounded = 128)]
    pub metadata: String,
    pub integrity: f32,
    #[protocol_attribute(protocol_type = VarLong)]
    pub seed: i64,
    pub flags: StructureBlockFlags,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum StructureBlockAction {
    UpdateData,
    SaveStructure,
    LoadStructure,
    DetectSize,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum MirrorMode {
    None,
    LeftRight,
    FrontBack,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum RotationMode {
    None,
    Clockwise90,
    Clockwise180,
    CounterClockwise90,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct StructureBlockFlags {
    pub ignore_entities: bool,
    pub show_air: bool,
    pub show_bounding_box: bool,
    pub strict: bool,
    #[bits(4)]
    __: u8,
}
