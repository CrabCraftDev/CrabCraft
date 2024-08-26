use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::hand::Hand;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x36, state = Play)]
pub struct SwingArmC2SPacket {
    pub hand: Hand,
}
