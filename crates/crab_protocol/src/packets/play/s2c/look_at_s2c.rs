use strum::FromRepr;

use crab_protocol::types::position::Position;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3D, state = Play)]
pub struct LookAtS2CPacket {
    pub aims_using: AimType,
    pub position: Position,
    pub entity: Option<AimEntity>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum AimType {
    Feet = 0,
    Eyes = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType)]
pub struct AimEntity {
    pub entity_id: VarInt,
    pub aims_at: AimType,
}
