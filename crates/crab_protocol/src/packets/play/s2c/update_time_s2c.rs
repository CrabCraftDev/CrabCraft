use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_time", id = 0x6B, state = Play)]
pub struct UpdateTimeS2CPacket {
    pub world_age: i64,
    pub time_of_day: i64,
}
