use strum::FromRepr;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x08, state = Play)]
pub struct ClientStatusC2SPacket {
    pub action: ClientStatusAction,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum ClientStatusAction {
    PerformRespawn,
    RequestStats,
}
