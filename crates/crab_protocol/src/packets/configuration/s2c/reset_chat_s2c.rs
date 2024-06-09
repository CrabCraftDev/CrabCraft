use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x06, state = Configuration)]
pub struct ResetChatS2CPacket;
