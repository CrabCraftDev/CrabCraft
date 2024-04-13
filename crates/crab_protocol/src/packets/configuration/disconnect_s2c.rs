use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::text::TextComponent;
use crab_protocol_macros::Packet;

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x01, state = Configuration)]
pub struct DisconnectS2CPacket {
    pub reason: TextComponent,
}
