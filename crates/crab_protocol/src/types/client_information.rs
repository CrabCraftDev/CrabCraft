use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol_macros::ProtocolSerializer;

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
