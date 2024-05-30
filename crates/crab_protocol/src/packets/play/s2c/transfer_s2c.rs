use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x73, state = Play)]
pub struct TransferS2CPacket {
    pub host: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub port: i32,
}
