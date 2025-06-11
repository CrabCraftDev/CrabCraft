use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::position::Position;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_look_at", id = 0x40, state = Play)]
pub struct LookAtS2CPacket {
    pub aims_using: AimType,
    pub position: Position,
    pub entity: Option<AimEntity>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum AimType {
    Feet = 0,
    Eyes = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer)]
pub struct AimEntity {
    pub entity_id: VarInt,
    pub aims_at: AimType,
}
