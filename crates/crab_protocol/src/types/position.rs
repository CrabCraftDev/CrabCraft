use crab_macros::ProtocolSerializer;

#[derive(Clone, Copy, PartialEq, Debug, ProtocolSerializer)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
