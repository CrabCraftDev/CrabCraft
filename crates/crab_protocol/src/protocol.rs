use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketDirection {
    /// Server -> Client
    Clientbound,
    /// Client -> Server
    Serverbound,
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
