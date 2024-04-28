use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::varlong::VarLong;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x31, state = Play)]
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

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum StructureBlockAction {
    UpdateData,
    SaveStructure,
    LoadStructure,
    DetectSize,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum MirrorMode {
    None,
    LeftRight,
    FrontBack,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum RotationMode {
    None,
    Clockwise90,
    Clockwise180,
    CounterClockwise90,
}

#[bitfield(u8)]
pub struct StructureBlockFlags {
    pub ignore_entities: bool,
    pub show_air: bool,
    pub show_bounding_box: bool,
    #[bits(5)]
    __: u8,
}

impl ProtocolType for StructureBlockFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self(buf.get_i8() as u8))
    }
}
