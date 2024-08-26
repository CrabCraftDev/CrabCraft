use bytes::Bytes;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crate::types::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct StoreCookieS2CPacket {
    pub key: NamespacedKey,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Bytes,
}

impl Packet for StoreCookieS2CPacket {
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x6B), (Configuration, 0x0A)];
}
