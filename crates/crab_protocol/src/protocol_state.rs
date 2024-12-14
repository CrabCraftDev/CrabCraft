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
    pub fn should_entity_be_spawned(&self) -> bool {
        *self == ProtocolState::Configuration || *self == ProtocolState::Play
    }
}
