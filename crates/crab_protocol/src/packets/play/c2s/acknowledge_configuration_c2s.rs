use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x0B, state = Play)]
pub struct AcknowledgeConfigurationC2SPacket;
