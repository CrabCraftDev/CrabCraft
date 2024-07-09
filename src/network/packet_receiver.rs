use bytes::BytesMut;
use crab_protocol::{
    error::ProtocolError, packet::Packet, packets::common::KeepAliveResponseC2SPacket, protocol_state::ProtocolState,
    raw_packet::RawPacket,
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf};

use crate::{
    network::handlers::{
        handshake_handler::HandshakeHandler, login_handler::LoginHandler, play_handler::PlayHandler,
        status_handler::StatusHandler,
    },
    utils::errors::Error,
};

use super::{connection::NetworkConnection, packet_splitter::PacketSplitter, packet_transformer::PacketTransformer};

// TODO: CONFIG_SYSTEM
const READ_BUFFER_SIZE: usize = 4096;

pub struct PacketReceiver {
    pub read_half: OwnedReadHalf,
    pub transformer: PacketTransformer,
    pub splitter: PacketSplitter,
    pub connection: NetworkConnection,
    pub status_handler: StatusHandler,
    pub handshake_handler: HandshakeHandler,
    pub login_handler: LoginHandler,
    pub play_handler: PlayHandler,
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
            play_handler: PlayHandler::new(connection),
        }
    }

    pub async fn receive_packets(&mut self) -> Result<Option<Vec<RawPacket>>, Error> {
        let mut buffer = BytesMut::with_capacity(READ_BUFFER_SIZE);

        if let Ok(0) = self.read_half.read_buf(&mut buffer).await {
            return Ok(None);
        }

        let bytes_vec = self.splitter.split(buffer)?;

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
            ProtocolState::Configuration => self.connection.incoming_packets_sender.send(raw_packet).unwrap(),
            ProtocolState::Play => {
                if raw_packet.id == KeepAliveResponseC2SPacket::get_id_for_state(ProtocolState::Play).unwrap() {
                    self.play_handler.handle_packet(raw_packet);
                } else {
                    self.connection.incoming_packets_sender.send(raw_packet).unwrap();
                }
            }
        };
    }
}
