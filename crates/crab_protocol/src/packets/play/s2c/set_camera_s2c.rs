use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_camera", id = 0x57, state = Play)]
pub struct SetCameraS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub camera_entity_id: i32,
}
