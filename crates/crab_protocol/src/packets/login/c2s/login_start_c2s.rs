use uuid::Uuid;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x00, state = Login)]
pub struct LoginStartC2SPacket {
    #[protocol_attribute(bounded = 16)]
    pub username: String,
    pub player_uuid: Uuid,
}
