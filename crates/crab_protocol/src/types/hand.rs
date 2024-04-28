use crab_protocol_macros::ProtocolType;
use strum::FromRepr;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum Hand {
    MainHand,
    OffHand,
}
