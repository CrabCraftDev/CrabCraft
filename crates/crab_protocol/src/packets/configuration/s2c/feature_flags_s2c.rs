use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x08, state = Configuration)]
pub struct FeatureFlagsS2CPacket {
    pub feature_flags: Vec<Identifier>,
}
