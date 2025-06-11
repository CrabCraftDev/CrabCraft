use bytes::Bytes;

use crab_macros::ProtocolSerializer;

use crate::packet::Packet;
use crate::protocol_state::ProtocolState;
use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct StoreCookieS2CPacket {
    pub key: NamespacedKey,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Bytes,
}

impl Packet for StoreCookieS2CPacket {
    const NAME: &'static str = "store_cookie";
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Play, 0x71), (Configuration, 0x0A)];
}
