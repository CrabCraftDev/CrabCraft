use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x14, state = Play)]
pub struct SetContainerPropertyS2CPacket {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}
