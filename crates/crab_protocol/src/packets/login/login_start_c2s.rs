use crate::protocol::ProtocolState::*;
use uuid::Uuid;

use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x00, state = Login)]
pub struct LoginStartC2SPacket {
    #[protocol_attribute(bounded = 16)]
    pub name: String,
    pub player_uuid: Uuid,
}
