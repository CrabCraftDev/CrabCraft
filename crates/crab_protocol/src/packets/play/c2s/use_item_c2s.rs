use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::hand::Hand;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x3B, state = Play)]
pub struct UseItemC2SPacket {
    pub hand: Hand,
    #[protocol_attribute(protocol_type = VarInt)]
    pub sequence: i32,
    pub yaw: f32,
    pub pitch: f32,
}
