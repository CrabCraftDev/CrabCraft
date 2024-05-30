use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::identifier::Identifier;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Configuration)]
pub struct CookieResponseC2SPacket {
    pub key: Identifier,
    #[protocol_attribute(bounded = 5120)]
    pub payload: Option<Bytes>,
}
