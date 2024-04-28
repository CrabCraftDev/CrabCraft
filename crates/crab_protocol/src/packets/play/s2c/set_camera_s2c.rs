use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x50, state = Play)]
pub struct SetCameraS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub camera_entity_id: i32,
}
