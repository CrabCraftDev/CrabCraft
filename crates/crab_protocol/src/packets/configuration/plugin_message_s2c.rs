use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bytes::LastBytes;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Configuration)]
pub struct PluginMessageS2CPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 1048576)]
    pub data: LastBytes,
}
