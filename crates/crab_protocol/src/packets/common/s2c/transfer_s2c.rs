use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct TransferS2CPacket {
    pub host: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub port: i32,
}

impl Packet for TransferS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x7A), (Configuration, 0x0B)];
}
