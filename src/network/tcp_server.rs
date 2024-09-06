use crate::network::connection::Connection;
use crate::network::early_connection_handler::EarlyConnectionHandler;
use crate::network::packet_transformer::PacketTransformer;
use crab_protocol::buf::get_var_int_from_stream;
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use kanal::Sender;
use log::{debug, error};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::process::exit;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    select,
};

const MAX_PACKET_SIZE_BYTES: i32 = (1 << 21) - 1;

pub struct TcpServer {
    address: String,
    /// Used to send Player Connection to MC Server for further processing.
    new_player_sender: Sender<Connection>,
}

impl TcpServer {
    pub fn new(address: String, new_player_sender: Sender<Connection>) -> TcpServer {
        TcpServer {
            address,
            new_player_sender,
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap_or_else(|error| {
            let error_message = match error.kind() {
                ErrorKind::AddrInUse => "Tried to bind to an address that's already in use!",
                ErrorKind::PermissionDenied => "Tried to bind to a protected address!",
                _ => panic!("{error}"),
            };
            error!("Failed to bind TCP server to {}! {error_message}", self.address);
            exit(1);
        });

        self.handle_incoming_connections(listener).await;
    }

    async fn handle_incoming_connections(&self, listener: TcpListener) {
        loop {
            match listener.accept().await {
                Ok((stream, address)) => {
                    debug!("Connection established {address}!");
                    stream.set_nodelay(true).unwrap();
                    tokio::spawn(Self::handle_connection(self.new_player_sender.clone(), stream, address));
                }
                Err(err) => {
                    error!("Error accepting connection: {err}");
                }
            }
        }
    }

    async fn handle_connection(new_player_sender: Sender<Connection>, stream: TcpStream, address: SocketAddr) {
        let (mut read_half, mut write_half) = tokio::io::split(stream);

        let (outgoing_packets_sender, outgoing_packets_receiver) = kanal::unbounded::<RawPacket>();
        let (incoming_packets_sender, incoming_packets_receiver) = kanal::unbounded::<RawPacket>();

        let mut connection = Connection::new(address, outgoing_packets_sender, incoming_packets_receiver);
        let packet_transformer = PacketTransformer::default();

        // Disconnect any connection that is in early state for longer than 500ms
        let mut connection_clone = connection.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            if connection_clone.state().is_early() {
                connection_clone.close();
            }
        });

        let packet_transformer_clone = packet_transformer.clone();
        let connection_clone = connection.clone();
        let writer = tokio::spawn(async move {
            while let Ok(packet) = outgoing_packets_receiver.as_async().recv().await {
                let bytes = match packet_transformer_clone.encode(packet, connection_clone.state()) {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        error!("Serialization error: {}", err);
                        outgoing_packets_receiver.close();
                        break;
                    }
                };

                if write_half.write_all(&bytes).await.is_err() {
                    break;
                }
            }
        });

        let mut connection_clone = connection.clone();
        let reader = tokio::spawn(async move {
            while let Ok(len) = get_var_int_from_stream(&mut read_half).await {
                if len > MAX_PACKET_SIZE_BYTES {
                    connection_clone.disconnect("Received too large packet!");
                    break;
                }
                if connection_clone.is_closed() {
                    break;
                }

                let mut buffer = vec![0; len as usize];
                if let Ok(0) = read_half.read_exact(&mut buffer).await {
                    break;
                }
                let raw_packet = packet_transformer.decode(buffer).unwrap();

                match connection_clone.state() {
                    ProtocolState::Status => {
                        EarlyConnectionHandler::handle_status_packet(&mut connection_clone, raw_packet)
                    }
                    ProtocolState::Handshake => EarlyConnectionHandler::handle_handshake_packet(
                        &mut connection_clone,
                        raw_packet,
                        new_player_sender.clone(),
                    ),
                    _ => incoming_packets_sender.send(raw_packet).unwrap(),
                }
            }
        });

        select! {
            _ = writer => (),
            _ = reader => ()
        }

        connection.close()
    }
}
