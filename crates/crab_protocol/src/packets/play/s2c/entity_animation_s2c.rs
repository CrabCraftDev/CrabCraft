use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x03, state = Play)]
pub struct EntityAnimationS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub animation: EntityAnimation,
}

#[derive(Debug, Copy, Clone, PartialEq, FromRepr, ProtocolSerializer)]
#[repr(u8)]
pub enum EntityAnimation {
    SwingMainArm = 0,
    LeaveBed = 2,
    SwingOtherArm = 3,
    CriticalEffect = 4,
    MagicalCriticalEffect = 5,
}
