use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "container_set_data", id = 0x14, state = Play)]
pub struct SetContainerPropertyS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub property: i16,
    pub value: i16,
}
