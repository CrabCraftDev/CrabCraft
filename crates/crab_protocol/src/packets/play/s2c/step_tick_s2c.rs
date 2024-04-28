use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x6F, state = Play)]
pub struct StepTickS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub tick_steps: i32,
}
