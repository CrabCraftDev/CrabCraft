use bytes::Bytes;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x01, state = Login)]
pub struct EncryptionResponseC2SPacket {
    pub shared_secret: Bytes,
    pub verify_token: Bytes,
}
