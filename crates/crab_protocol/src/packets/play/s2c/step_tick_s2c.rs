use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "ticking_step", id = 0x79, state = Play)]
pub struct StepTickS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub tick_steps: i32,
}
