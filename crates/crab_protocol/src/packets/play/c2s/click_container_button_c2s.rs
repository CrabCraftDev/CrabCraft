use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0F, state = Play)]
pub struct ClickContainerButtonC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub button_id: i8,
}
