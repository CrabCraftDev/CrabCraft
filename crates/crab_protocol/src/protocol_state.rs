use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
