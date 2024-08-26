use crab_macros::ProtocolSerializer;
use strum::FromRepr;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum SampleType {
    TickTime,
}
