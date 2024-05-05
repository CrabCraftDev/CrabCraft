use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x6F, state = Play)]
pub struct StepTickS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub tick_steps: i32,
}
