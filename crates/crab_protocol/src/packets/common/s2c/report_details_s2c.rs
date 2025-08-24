use crab_macros::{Packet, ProtocolSerializer};
use crab_protocol::protocol_state::ProtocolState::{Configuration, Play};

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "custom_report_details", ids(Play = 0x81, Configuration = 0x0F))]
pub struct ReportDetailsS2CPacket {
    #[protocol_attribute(bounded = 32)]
    pub details: Vec<ReportDetail>,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ReportDetail {
    #[protocol_attribute(bounded = 128)]
    pub title: String,
    #[protocol_attribute(bounded = 4096)]
    pub description: String,
}
