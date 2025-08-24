use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::text::Text;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "resource_pack_push", ids(Configuration = 0x09, Play = 0x4A))]
pub struct AddResourcePackS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub url: String,
    #[protocol_attribute(bounded = 40)]
    /// SHA-1 hash of the resource pack
    pub hash: String,
    pub is_forced: bool,
    pub prompt_message: Option<Text>,
}
