use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::client_information::{ChatMode, DisplayedSkinParts, MainHand};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
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
