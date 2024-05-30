use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0A, state = Login)]
pub struct StoreCookieS2CPacket {
    pub key: Identifier,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Bytes,
}
