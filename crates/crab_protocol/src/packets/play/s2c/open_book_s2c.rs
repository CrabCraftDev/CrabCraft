use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

use crate::types::hand::Hand;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x30, state = Play)]
pub struct OpenBookS2CPacket {
    pub hand: Hand,
}
