use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::hand::Hand;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x32, state = Play)]
pub struct OpenBookS2CPacket {
    pub hand: Hand,
}
