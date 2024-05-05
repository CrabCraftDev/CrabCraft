use bytes::Bytes;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x01, state = Login)]
pub struct EncryptionRequestS2CPacket {
    #[protocol_attribute(bounded = 20)]
    pub server_id: String,
    pub public_key: Bytes,
    pub verify_token: Bytes,
}
