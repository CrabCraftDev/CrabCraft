use std::collections::HashMap;

use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;
use crate::types::text::TextComponent;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x7B, state = Play)]
pub struct UpdateAdvancementsS2CPacket {
    pub should_reset: bool,
    pub advancement_mapping: HashMap<NamespacedKey, Advancement>,
    pub namespaced_keys: Vec<NamespacedKey>,
    pub progress_mapping: HashMap<NamespacedKey, AdvancementProgress>,
}

#[derive(ProtocolSerializer, Clone, Debug)]
pub struct Advancement {
    pub parent_id: Option<NamespacedKey>,
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
    pub background_texture: Option<NamespacedKey>,
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
    pub progress: HashMap<NamespacedKey, AdvancementProgressCriterion>,
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
