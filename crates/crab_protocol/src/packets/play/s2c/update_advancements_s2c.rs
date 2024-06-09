use std::collections::HashMap;

use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x74, state = Play)]
pub struct UpdateAdvancementsS2CPacket {
    pub should_reset: bool,
    pub advancement_mapping: HashMap<Identifier, Advancement>,
    pub identifiers: Vec<Identifier>,
    pub progress_mapping: HashMap<Identifier, AdvancementProgress>,
}

#[derive(ProtocolSerializer, Clone, Debug)]
pub struct Advancement {
    pub parent_id: Option<Identifier>,
    pub display_data: Option<AdvancementDisplay>,
    pub requirements: Vec<Vec<String>>,
    pub sends_telemetry_data: bool,
}

#[derive(ProtocolSerializer, Clone, Debug)]
pub struct AdvancementDisplay {
    pub title: TextComponent,
    pub description: TextComponent,
    pub frame_type: AdvancementFrameType,
    pub flags: AdvancementDisplayFlags,
    pub background_texture: Option<Identifier>,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}

#[derive(ProtocolSerializer, Clone, PartialEq, Eq, Debug)]
pub struct AdvancementProgress {
    pub progress: HashMap<Identifier, AdvancementProgressCriterion>,
}

#[derive(ProtocolSerializer, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AdvancementProgressCriterion {
    pub achieved: bool,
    pub date_of_achieving: Option<i64>,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct AdvancementDisplayFlags {
    pub has_background_texture: bool,
    pub show_toast: bool,
    pub hidden: bool,
    #[bits(5)]
    __: u8,
}
