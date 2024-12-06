use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "status_request", id = 0x00, state = Status)]
pub struct StatusRequestC2SPacket;
