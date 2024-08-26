use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x03, state = Configuration)]
pub struct AcknowledgeFinishConfigurationC2SPacket;
