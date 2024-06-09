use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use uuid::Uuid;

use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::text::TextComponent;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct AddResourcePackS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub url: String,
    #[protocol_attribute(bounded = 40)]
    /// SHA-1 hash of the resource pack
    pub hash: String,
    pub is_forced: bool,
    pub prompt_message: Option<TextComponent>,
}

impl Packet for AddResourcePackS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x09), (Play, 0x46)];
}
