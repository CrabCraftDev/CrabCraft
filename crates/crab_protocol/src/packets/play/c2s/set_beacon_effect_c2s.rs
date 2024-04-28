use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2B, state = Play)]
pub struct SetBeaconEffectC2SPacket {
    #[protocol_attribute(protocol_type = Option<VarInt>)]
    pub primary_effect: Option<i32>,
    #[protocol_attribute(protocol_type = Option<VarInt>)]
    pub secondary_effect: Option<i32>,
}
