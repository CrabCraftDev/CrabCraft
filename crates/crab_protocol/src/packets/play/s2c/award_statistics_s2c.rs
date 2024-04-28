use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Debug, Packet, Clone)]
#[packet(id = 0x04, state = Play)]
pub struct AwardStatisticsS2CPacket {
    pub statistics: Vec<Statistic>,
}

#[derive(Debug, Clone, PartialEq, ProtocolType)]
pub struct Statistic {
    pub category_id: i32,
    pub statistic_id: i32,
    pub value: i32,
}
