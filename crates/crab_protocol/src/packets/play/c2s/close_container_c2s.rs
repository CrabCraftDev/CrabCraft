use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "container_close", id = 0x12, state = Play)]
pub struct CloseContainerC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
}
