use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "award_stats", id = 0x04, state = Play)]
pub struct AwardStatisticsS2CPacket {
    pub statistics: Vec<Statistic>,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct Statistic {
    pub category_id: i32,
    pub statistic_id: i32,
    pub value: i32,
}
