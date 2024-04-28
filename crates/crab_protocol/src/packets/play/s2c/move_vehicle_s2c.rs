use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::angle::Angle;
use crab_protocol::types::position::Position;
use crab_protocol_macros::Packet;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x2F, state = Play)]
pub struct MoveVehicleS2CPacket {
    pub position: Position,
    pub yaw: Angle,
    pub pitch: Angle,
}
