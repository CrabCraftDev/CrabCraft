use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x51, state = Play)]
pub struct SetHeldItemS2CPacket {
    pub slot: i8,
}
