use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PluginMessageS2CPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}

impl Packet for PluginMessageS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x01), (Play, 0x19)];
}
