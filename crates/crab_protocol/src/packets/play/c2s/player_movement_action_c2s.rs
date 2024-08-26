use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x25, state = Play)]
pub struct PlayerMovementActionC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub player_id: i32,
    pub command: PlayerMovementAction,
    #[protocol_attribute(protocol_type = VarInt)]
    /// Used only in Start jump with horse
    pub jump_boost: i32,
}

#[derive(ProtocolSerializer, FromRepr, Copy, Clone, Debug)]
pub enum PlayerMovementAction {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHose,
    StopJumpWithHorse,
    OpenVehicleInventory,
    StartFlyingWithElytra,
}
