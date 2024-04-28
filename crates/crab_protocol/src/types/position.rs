use crab_protocol_macros::ProtocolType;

#[derive(Clone, Copy, PartialEq, Debug, ProtocolType)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
