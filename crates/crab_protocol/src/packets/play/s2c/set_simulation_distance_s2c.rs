use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "set_simulation_distance", id = 0x68, state = Play)]
pub struct SetSimulationDistanceS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub simulation_distance: i32,
}
