use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::bytes::LastBytes;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x02, state = Login)]
pub struct LoginPluginResponseC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub message_id: i32,
    #[protocol_attribute(bounded = 1048576)]
    pub data: LastBytes,
}
