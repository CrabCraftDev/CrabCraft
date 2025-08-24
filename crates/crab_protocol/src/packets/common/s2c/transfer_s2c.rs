use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "transfer", ids(Play = 0x7A, Configuration = 0x0B))]
pub struct TransferS2CPacket {
    pub host: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub port: i32,
}
