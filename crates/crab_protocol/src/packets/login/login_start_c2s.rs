use uuid::Uuid;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Login)]
pub struct LoginStartC2SPacket {
    #[protocol_attribute(bounded = 16)]
    pub username: String,
    pub player_uuid: Uuid,
}
