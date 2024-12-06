use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "start_configuration", id = 0x70, state = Play)]
pub struct StartConfigurationS2CPacket;
