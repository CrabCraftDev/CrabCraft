use uuid::Uuid;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x07, state = Configuration)]
pub struct AddResourcePackS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub url: String,
    #[protocol_attribute(bounded = 40)]
    /// SHA-1 hash of the resource pack
    pub hash: String,
    pub is_forced: bool,
    pub prompt_message: Option<TextComponent>,
}
