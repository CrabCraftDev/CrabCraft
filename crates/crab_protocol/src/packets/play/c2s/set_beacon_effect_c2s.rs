use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_beacon", id = 0x32, state = Play)]
pub struct SetBeaconEffectC2SPacket {
    #[protocol_attribute(protocol_type = Option<VarInt>)]
    pub primary_effect: Option<i32>,
    #[protocol_attribute(protocol_type = Option<VarInt>)]
    pub secondary_effect: Option<i32>,
}
