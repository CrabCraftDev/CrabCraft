use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x0A, state = Play)]
pub struct ClientStatusC2SPacket {
    pub action: ClientStatusAction,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum ClientStatusAction {
    PerformRespawn,
    RequestStats,
}
