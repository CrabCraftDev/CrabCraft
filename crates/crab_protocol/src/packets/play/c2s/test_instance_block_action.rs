use strum::FromRepr;

use crate::protocol_state::ProtocolState::*;
use crate::types::block_position::BlockPosition;
use crate::types::position::{Rotation, Vec3Int};
use crab_macros::{Packet, ProtocolSerializer};
use crab_text::text::Text;
use crab_text::utils::namespaced_key::NamespacedKey;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "test_instance_block_action", id = 0x3E, state = Play)]
pub struct TestInstanceBlockActionC2SPacket {
    pub position: BlockPosition,
    pub action: TestBlockAction,
    pub data: TestBlockData,
}

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct TestBlockData {
    pub namespaced_key: Option<NamespacedKey>,
    pub size: Vec3Int,
    pub rotation: Rotation,
    pub ignore_entities: bool,
    pub status: TestBlockStatus,
    pub error_message: Option<Text>,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum TestBlockStatus {
    Cleared,
    Running,
    Finished,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum TestBlockAction {
    Init,
    Query,
    Set,
    Reset,
    Save,
    Export,
    Run,
}
