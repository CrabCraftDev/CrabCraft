use std::cmp::PartialEq;
use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

impl ProtocolState {
    pub fn is_early(&self) -> bool {
        *self == ProtocolState::Handshake || *self == ProtocolState::Status
    }
}
