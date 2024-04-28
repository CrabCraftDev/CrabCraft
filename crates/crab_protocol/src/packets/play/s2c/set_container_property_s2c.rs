use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x14, state = Play)]
pub struct SetContainerPropertyS2CPacket {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}
