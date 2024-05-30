use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::bytes::EndBytes;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x19, state = Play)]
pub struct PluginMessageS2CPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 1048576)]
    pub data: EndBytes,
}
