use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0B, state = Login)]
pub struct TransferS2CPacket {
    pub host: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub port: i32,
}
