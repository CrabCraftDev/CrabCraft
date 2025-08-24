use strum::FromRepr;
use uuid::Uuid;

use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "resource_pack", ids(Configuration = 0x06, Play = 0x30))]
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
