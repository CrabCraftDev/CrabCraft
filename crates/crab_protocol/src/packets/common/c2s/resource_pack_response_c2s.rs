use crate::packet::Packet;
use crate::protocol::{PacketDirection, ProtocolState};
use strum::FromRepr;
use uuid::Uuid;

use crab_protocol_macros::ProtocolSerializer;

use crate::protocol::ProtocolState::*;

#[derive(ProtocolSerializer, Debug, Clone)]
pub struct ResourcePackResponseC2SPacket {
    pub resource_pack_uuid: Uuid,
    pub result: ResourcePackStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum ResourcePackStatus {
    SuccessfullyDownloaded,
    Declined,
    FailedToDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedToReload,
    Discarded,
}

impl Packet for ResourcePackResponseC2SPacket {
    const SIDE: PacketDirection = PacketDirection::Serverbound;
    const STATE_IDS: &'static [(ProtocolState, i32)] = &[(Configuration, 0x06), (Play, 0x2B)];
}
