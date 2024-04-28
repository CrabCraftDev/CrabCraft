use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x53, state = Play)]
pub struct SetRenderDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub view_distance: i32,
}
