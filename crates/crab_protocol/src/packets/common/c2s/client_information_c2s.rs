use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use bitfield_struct::bitfield;
use crab_protocol_macros::ProtocolSerializer;
use strum::FromRepr;

use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ClientInformationC2SPacket {
    #[protocol_attribute(bounded = 16)]
    pub locale: String,
    pub render_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors_enabled: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum MainHand {
    Left,
    Right,
}

#[bitfield(u8)]
#[derive(ProtocolSerializer)]
pub struct DisplayedSkinParts {
    pub cape_enabled: bool,
    pub jacket_enabled: bool,
    pub left_sleeve_enabled: bool,
    pub right_sleeve_enabled: bool,
    pub left_pants_leg_enabled: bool,
    pub right_pants_leg_enabled: bool,
    pub hat_enabled: bool,
    #[bits(1)]
    __: u8,
}

impl Packet for ClientInformationC2SPacket {
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x0A), (Configuration, 0x00)];
}
