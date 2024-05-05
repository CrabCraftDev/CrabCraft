use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x60, state = Play)]
pub struct SetSimulationDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub simulation_distance: i32,
}
