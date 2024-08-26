use strum::FromRepr;

use crab_macros::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum Hand {
    MainHand,
    OffHand,
}
