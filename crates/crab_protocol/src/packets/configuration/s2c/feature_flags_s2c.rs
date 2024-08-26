use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0C, state = Configuration)]
pub struct FeatureFlagsS2CPacket {
    pub feature_flags: Vec<Identifier>,
}
