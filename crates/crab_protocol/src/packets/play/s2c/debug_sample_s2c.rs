use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::sample_type::SampleType;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "debug_sample", id = 0x1A, state = Play)]
pub struct DebugSampleS2CPacket {
    pub sample: Vec<i64>,
    pub sample_type: SampleType,
}
