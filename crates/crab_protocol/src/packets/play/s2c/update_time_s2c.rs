use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x64, state = Play)]
pub struct UpdateTimeS2CPacket {
    pub world_age: i64,
    pub time_of_day: i64,
}
