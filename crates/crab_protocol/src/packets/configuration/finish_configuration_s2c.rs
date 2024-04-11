use crate::protocol::ProtocolState::*;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x02, state = Configuration)]
pub struct FinishConfigurationS2CPacket;
