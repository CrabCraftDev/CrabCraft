use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "finish_configuration", id = 0x03, state = Configuration)]
pub struct FinishConfigurationS2CPacket;
