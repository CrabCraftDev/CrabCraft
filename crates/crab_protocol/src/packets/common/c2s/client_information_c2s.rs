use bitfield_struct::bitfield;
use evenio::component::Component;
use strum::FromRepr;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;

#[derive(ProtocolSerializer, Component, Debug, Clone)]
pub struct ClientInformation {
    #[protocol_attribute(bounded = 16)]
    pub locale: String,
    pub render_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors_enabled: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: bool,
}

pub type ClientInformationC2SPacket = ClientInformation;

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
    const NAME: &'static str = "client_information";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x0D), (Configuration, 0x00)];
}
