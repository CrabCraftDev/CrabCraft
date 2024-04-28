use crab_protocol_macros::ProtocolType;
use strum::FromRepr;

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
