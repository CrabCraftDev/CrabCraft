use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;
use crate::types::block_face::BlockFace;
use crate::types::block_position::BlockPosition;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x24, state = Play)]
pub struct PlayerActionC2SPacket {
    pub status: PlayerAction,
    pub position: BlockPosition,
    pub face: BlockFace,
    #[protocol_attribute(protocol_type = VarInt)]
    /// 0 for all statuses except for start, cancelled, and finished digging.
    pub sequence_id: i32,
}

#[derive(ProtocolSerializer, FromRepr, Copy, Clone, Debug)]
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
