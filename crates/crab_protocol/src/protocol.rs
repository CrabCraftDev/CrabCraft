#[derive(Debug, PartialEq)]
pub enum PacketDirection {
    /// Server -> Client
    Clientbound,
    /// Client -> Server
    Serverbound,
}

#[derive(Debug, PartialEq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
