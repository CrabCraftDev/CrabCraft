use bytes::BytesMut;
use crab_protocol::{
    error::ProtocolError,
    packet::{self},
    packets::common::KeepAliveResponseC2SPacket,
    protocol_state::ProtocolState,
    raw_packet::RawPacket,
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

use crate::{
    network::handlers::{
        handshake_handler::HandshakeHandler, login_handler::LoginHandler, status_handler::StatusHandler,
    },
    utils::errors::Error,
};

use super::{
    connection::NetworkConnection, handlers::keep_alive_handler::KeepAliveHandler, packet_splitter::PacketSplitter,
    packet_transformer::PacketTransformer,
};

pub struct PacketReceiver {
    pub read_half: OwnedReadHalf,
    pub transformer: PacketTransformer,
    pub splitter: PacketSplitter,
    pub connection: NetworkConnection,
    pub status_handler: StatusHandler,
    pub handshake_handler: HandshakeHandler,
    pub login_handler: LoginHandler,
    pub keep_alive_handler: KeepAliveHandler,
}

impl PacketReceiver {
    pub fn new(read_half: OwnedReadHalf, transformer: PacketTransformer, connection: NetworkConnection) -> Self {
        Self {
            read_half,
            transformer,
            splitter: PacketSplitter::new(),
            connection: connection.clone(),
            status_handler: StatusHandler::new(connection.clone()),
            handshake_handler: HandshakeHandler::new(connection.clone()),
            login_handler: LoginHandler::new(connection.clone()),
            keep_alive_handler: KeepAliveHandler::new(connection),
        }
    }

    pub async fn receive_packets(&mut self, buffer: &mut BytesMut) -> Result<Option<Vec<RawPacket>>, Error> {
        if let Ok(0) = self.read_half.read_buf(buffer).await {
            return Ok(None);
        }

        let bytes_vec = self.splitter.split_bytes(buffer)?;

        let packets_vec = bytes_vec
            .into_iter()
            .map(|bytes| self.transformer.decode(bytes))
            .collect::<Result<Vec<RawPacket>, ProtocolError>>()?;

        Ok(Some(packets_vec))
    }

    pub async fn process_packet(&self, raw_packet: RawPacket) {
        let state = self.connection.get_state();
        match state {
            ProtocolState::Status => self.status_handler.handle_packet(raw_packet),
            ProtocolState::Handshake => self.handshake_handler.handle_packet(raw_packet),
            ProtocolState::Login => self.login_handler.handle_packet(raw_packet),
            ProtocolState::Configuration | ProtocolState::Play => {
                if raw_packet.id == packet::get_id_for_state::<KeepAliveResponseC2SPacket>(state).unwrap() {
                    self.keep_alive_handler.handle_keepalive(raw_packet);
                } else {
                    self.connection.incoming_packets_sender.send(raw_packet).unwrap();
                }
            }
        };
    }
}
