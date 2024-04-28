use crab_protocol::types::hand::Hand;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x33, state = Play)]
pub struct SwingArmC2SPacket {
    pub hand: Hand,
}
