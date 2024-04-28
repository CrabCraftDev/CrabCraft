use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::Packet;

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x60, state = Play)]
pub struct SetSimulationDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub simulation_distance: i32,
}
