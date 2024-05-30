use crab_protocol::types::sample_type::SampleType;
use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x13, state = Configuration)]
pub struct DebugSampleSubscriptionC2SPacket {
    pub sample_type: SampleType,
}
