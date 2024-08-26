use crab_macros::ProtocolSerializer;
use crab_protocol::packet::Packet;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::protocol_state::ProtocolState::{Configuration, Play};

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ReportDetailsS2CPacket {
    #[protocol_attribute(bounded = 32)]
    pub details: Vec<ReportDetail>,
}

impl Packet for ReportDetailsS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x7A), (Configuration, 0x0F)];
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ReportDetail {
    #[protocol_attribute(bounded = 128)]
    pub title: String,
    #[protocol_attribute(bounded = 4096)]
    pub description: String,
}
