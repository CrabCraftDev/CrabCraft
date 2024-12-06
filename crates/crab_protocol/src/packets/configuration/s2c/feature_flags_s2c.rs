use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "update_enabled_features", id = 0x0C, state = Configuration)]
pub struct FeatureFlagsS2CPacket {
    pub feature_flags: Vec<NamespacedKey>,
}
