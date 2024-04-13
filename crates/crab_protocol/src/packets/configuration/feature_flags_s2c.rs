use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::identifier::Identifier;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x08, state = Configuration)]
pub struct FeatureFlagsS2CPacket {
    pub feature_flags: Vec<Identifier>,
}
