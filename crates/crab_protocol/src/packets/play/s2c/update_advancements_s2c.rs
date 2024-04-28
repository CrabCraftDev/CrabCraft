use std::collections::HashMap;

use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x70, state = Play)]
pub struct UpdateAdvancementsS2CPacket {
    pub should_reset: bool,
    pub advancement_mapping: HashMap<Identifier, Advancement>,
    pub identifiers: Vec<Identifier>,
    pub progress_mapping: HashMap<Identifier, AdvancementProgress>,
}

#[derive(ProtocolType, Clone, Debug)]
pub struct Advancement {
    pub parent_id: Option<Identifier>,
    pub display_data: Option<AdvancementDisplay>,
    pub requirements: Vec<Vec<String>>,
    pub sends_telemetry_data: bool,
}

#[derive(ProtocolType, Clone, Debug)]
pub struct AdvancementDisplay {
    pub title: TextComponent,
    pub description: TextComponent,
    pub frame_type: AdvancementFrameType,
    pub flags: AdvancementDisplayFlags,
    pub background_texture: Option<Identifier>,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}

#[derive(ProtocolType, Clone, PartialEq, Eq, Debug)]
pub struct AdvancementProgress {
    pub progress: HashMap<Identifier, AdvancementProgressCriterion>,
}

#[derive(ProtocolType, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AdvancementProgressCriterion {
    pub achieved: bool,
    pub date_of_achieving: Option<i64>,
}

#[bitfield(u8)]
pub struct AdvancementDisplayFlags {
    pub has_background_texture: bool,
    pub show_toast: bool,
    pub hidden: bool,
    #[bits(5)]
    __: u8,
}

impl ProtocolType for AdvancementDisplayFlags {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_i8(self.0 as i8);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(Self(buf.get_i8() as u8))
    }
}
