use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "configuration_acknowledged", id = 0x0F, state = Play)]
pub struct AcknowledgeConfigurationC2SPacket;
