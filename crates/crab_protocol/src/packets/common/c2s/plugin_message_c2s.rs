use crate::packet::Packet;
use crate::protocol::PacketDirection;
use crate::protocol::ProtocolState;
use crab_protocol_macros::{ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::identifier::Identifier;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct PluginMessageC2SPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 32767)]
    pub data: EndBytes,
}

impl Packet for PluginMessageC2SPacket {
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x02), (Play, 0x12)];
}
