use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Configuration)]
pub struct PluginMessageC2SPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 32767)]
    pub data: EndBytes,
}
