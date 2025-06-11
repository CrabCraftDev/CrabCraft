use uuid::Uuid;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct AddResourcePackS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub url: String,
    #[protocol_attribute(bounded = 40)]
    /// SHA-1 hash of the resource pack
    pub hash: String,
    pub is_forced: bool,
    pub prompt_message: Option<Text>,
}

impl Packet for AddResourcePackS2CPacket {
    const NAME: &'static str = "resource_pack_push";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x09), (Play, 0x4A)];
}
