use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x50, state = Play)]
pub struct SetCameraS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub camera_entity_id: i32,
}
