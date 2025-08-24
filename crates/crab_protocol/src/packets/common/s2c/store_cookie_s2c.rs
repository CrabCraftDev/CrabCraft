use bytes::Bytes;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "store_cookie", ids(Play = 0x71, Configuration = 0x0A))]
pub struct StoreCookieS2CPacket {
    pub key: NamespacedKey,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Bytes,
}
