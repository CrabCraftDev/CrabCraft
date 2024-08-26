use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::sample_type::SampleType;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x1B, state = Play)]
pub struct DebugSampleS2CPacket {
    pub sample: Vec<i64>,
    pub sample_type: SampleType,
}
