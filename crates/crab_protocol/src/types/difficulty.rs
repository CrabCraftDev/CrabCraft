use strum::FromRepr;

use crab_macros::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, FromRepr, ProtocolSerializer)]
#[repr(u8)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
