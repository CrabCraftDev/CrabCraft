use strum::FromRepr;

use crab_protocol_macros::ProtocolSerializer;

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
