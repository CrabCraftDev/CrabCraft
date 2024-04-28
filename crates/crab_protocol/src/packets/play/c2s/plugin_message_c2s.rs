use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bytes::LastBytes;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x10, state = Play)]
pub struct PluginMessageC2SPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 32767)]
    pub data: LastBytes,
}
