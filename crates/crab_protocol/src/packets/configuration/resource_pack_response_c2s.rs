use strum::FromRepr;
use uuid::Uuid;

use crab_protocol::protocol::ProtocolState::*;
use crab_protocol_macros::{Packet, ProtocolType};

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x05, state = Configuration)]
pub struct ResourcePackResponseS2CPacket {
    pub resource_pack_uuid: Uuid,
    pub result: ResourcePackStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
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
