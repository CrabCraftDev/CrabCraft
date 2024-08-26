use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::sample_type::SampleType;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x13, state = Play)]
pub struct DebugSampleSubscriptionC2SPacket {
    pub sample_type: SampleType,
}
