use bitfield_struct::bitfield;
use strum::FromRepr;

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol_type::ProtocolType;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Configuration)]
pub struct ClientInformationC2SPacket {
    #[protocol_attribute(bounded = 16)]
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors_enabled: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum MainHand {
    Left,
    Right,
}

#[bitfield(u8)]
pub struct DisplayedSkinParts {
    pub cape_enabled: bool,
    pub jacket_enabled: bool,
    pub left_sleeve_enabled: bool,
    pub right_sleeve_enabled: bool,
    pub left_pants_leg_enabled: bool,
    pub right_pants_leg_enabled: bool,
    pub hat_enabled: bool,
    __: bool,
}

impl ProtocolType for DisplayedSkinParts {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_u8(self.0);
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(DisplayedSkinParts(buf.get_u8()))
    }
}
