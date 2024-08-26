use strum::FromRepr;

use crab_macros::ProtocolSerializer;

#[derive(Debug, Copy, Clone, PartialEq, FromRepr, ProtocolSerializer)]
#[repr(u8)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}
