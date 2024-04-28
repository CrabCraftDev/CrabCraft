use uuid::Uuid;

use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x44, state = Play)]
pub struct AddResourcePackS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub url: String,
    #[protocol_attribute(bounded = 40)]
    pub hash: String,
    pub is_forced: bool,
    pub prompt_message: Option<TextComponent>,
}
