use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::hand::Hand;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "swing", id = 0x3A, state = Play)]
pub struct SwingArmC2SPacket {
    pub hand: Hand,
}
