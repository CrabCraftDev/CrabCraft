use crate::protocol::ProtocolState::*;
use crate::types::bytes::LastBytes;
use crate::types::identifier::Identifier;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x01, state = Configuration)]
pub struct PluginMessageC2SPacket {
    pub channel: Identifier,
    #[protocol_attribute(bounded = 32767)]
    pub data: LastBytes,
}
