use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct TransferS2CPacket {
    pub host: String,
    #[protocol_attribute(protocol_type = VarInt)]
    pub port: i32,
}

impl Packet for TransferS2CPacket {
    const SIDE: PacketDirection = PacketDirection::Clientbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x73), (Configuration, 0x0B)];
}
