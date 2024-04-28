use strum::FromRepr;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x22, state = Play)]
pub struct PlayerMovementActionC2SPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub player_id: i32,
    pub command: PlayerMovementAction,
    #[protocol_attribute(protocol_type = VarInt)]
    /// Used only in Start jump with horse
    pub jump_boost: i32,
}

#[derive(ProtocolType, FromRepr, Copy, Clone, Debug)]
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
