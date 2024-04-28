use strum::FromRepr;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol::types::varint::VarInt;
use crab_protocol_macros::{Packet, ProtocolType};

use crate::types::block_face::BlockFace;
use crate::types::block_position::BlockPosition;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x21, state = Play)]
pub struct PlayerActionC2SPacket {
    pub status: PlayerAction,
    pub position: BlockPosition,
    pub face: BlockFace,
    #[protocol_attribute(protocol_type = VarInt)]
    /// 0 for all statuses except for start, cancelled, and finished digging.
    pub sequence_id: i32,
}

#[derive(ProtocolType, FromRepr, Copy, Clone, Debug)]
pub enum PlayerAction {
    StartedDigging,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropItem,
    /// Shoot arrow, eat food etc.
    UpdateHeldItemState,
    SwapItemInHand,
}
