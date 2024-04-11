use crate::protocol::ProtocolState::*;
use bytes::Bytes;
use crab_protocol::types::identifier::Identifier;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Configuration)]
pub struct PluginMessageS2CPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 1048576)]
    pub data: Bytes,
}
