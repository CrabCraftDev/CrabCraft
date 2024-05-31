use std::sync::Arc;

use bytes::Bytes;
use chrono::Local;
use crossbeam::sync::ShardedLock;
use kanal::Sender;
use log::{debug, error, warn};
use tokio::io::ReadHalf;
use tokio::sync::Mutex;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::network::connection::Connection;
use crate::{
    entity::player::network::connection::PlayerConnection, network::handlers::handshake_handler::HandshakeHandler,
};

use crate::network::handlers::login_handler::LoginHandler;
use crate::network::handlers::play_handler::PlayHandler;
use crate::network::handlers::status_handler::StatusHandler;
use crab_protocol::raw_packet::RawPacket;
use crab_protocol::{
    buf::{get_var_int_from_stream, ReadByteBuf},
    packets::play::KeepAliveResponseC2SPacket,
};
use crab_protocol::{packet::Packet, protocol::ProtocolState};

pub struct TcpServer {
    address: String,
    pub connections: Arc<Mutex<Vec<Arc<Mutex<Connection>>>>>,
    new_player_sender: Sender<PlayerConnection>,
}

impl TcpServer {
    pub fn new(address: String, new_player_sender: Sender<PlayerConnection>) -> TcpServer {
        TcpServer {
            address,
            connections: Arc::new(Mutex::new(Vec::new())),
            new_player_sender,
        }
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();

        loop {
            match listener.accept().await {
                Ok((stream, address)) => {
                    debug!("Connection established {address}!");

                    stream.set_nodelay(true).unwrap();
                    let (read_half, write_half) = tokio::io::split(stream);

                    let connection = Arc::new(Mutex::new(Connection {
                        write_half,
                        state: Arc::new(ShardedLock::new(ProtocolState::Handshake)),
                        is_closed: false,
                        address,
                        keepalive_pending: false,
                        last_keepalive_time: Local::now().timestamp_millis(),
                    }));
                    self.connections.lock().await.push(connection.clone());
                    let connections_mutex = Arc::clone(&self.connections);
                    let sender = self.new_player_sender.clone();
                    tokio::spawn(async move {
                        TcpServer::handle_connection(connection, read_half, sender).await;
                        connections_mutex
                            .lock()
                            .await
                            .retain(|conn| !conn.try_lock().unwrap().is_closed)
                    });
                }
                Err(err) => {
                    error!("Error accepting connection: {err}");
                }
            }
        }
    }

    pub async fn tick(&self) {
        let connections = self.connections.lock().await;
        for conn in connections.iter() {
            let mut conn = conn.lock().await;
            conn.keep_connection_alive().await;
        }
    }

    async fn handle_connection(
        connection: Arc<Mutex<Connection>>,
        mut read_half: ReadHalf<TcpStream>,
        new_player_sender: Sender<PlayerConnection>,
    ) {
        let (incoming_packets_sender, incoming_packets_receiver) = kanal::unbounded::<RawPacket>();
        let (outgoing_packets_sender, outgoing_packets_receiver) = kanal::unbounded::<RawPacket>();

        let connection_clone = Arc::clone(&connection);
        tokio::spawn(async move {
            let receiver = outgoing_packets_receiver.to_async();
            while let Ok(packet) = receiver.recv().await {
                connection_clone.lock().await.send_raw_packet(packet).await;
            }
        });

        while let Ok(len) = get_var_int_from_stream(&mut read_half).await {
            let mut buffer = vec![0; len as usize];

            if let Ok(0) = read_half.read_exact(&mut buffer).await {
                break;
            }

            let mut bytes = Bytes::from(buffer);

            let packet_id = bytes.get_var_int().unwrap();
            let raw_packet = RawPacket::new(packet_id, bytes);

            let mut connection = connection.lock().await;
            let state = *connection.state.read().unwrap();
            let result = match state {
                ProtocolState::Status => StatusHandler::handle_packet(&mut connection, raw_packet).await,
                ProtocolState::Handshake => HandshakeHandler::handle_packet(&mut connection, raw_packet).await,
                ProtocolState::Login => match LoginHandler::handle_packet(&mut connection, raw_packet).await {
                    Ok(true) => {
                        new_player_sender
                            .send(PlayerConnection {
                                protocol_state: Arc::clone(&connection.state),
                                packet_receiver: incoming_packets_receiver.clone(),
                                packet_sender: outgoing_packets_sender.clone(),
                            })
                            .unwrap();
                        Ok(())
                    }
                    Ok(false) => Ok(()),
                    Err(err) => Err(err),
                },
                ProtocolState::Configuration => {
                    incoming_packets_sender.send(raw_packet).unwrap();
                    Ok(())
                }
                ProtocolState::Play => {
                    if raw_packet.id == KeepAliveResponseC2SPacket::ID {
                        PlayHandler::handle_packet(&mut connection, raw_packet).await
                    } else {
                        incoming_packets_sender.send(raw_packet).unwrap();
                        Ok(())
                    }
                }
            };

            if result.is_err() {
                warn!("Error handling packet {:?}!", result.err());
                connection.disconnect("Unexpected error!").await;
                break;
            }
        }

        let mut connection = connection.lock().await;
        connection.is_closed = true;
        debug!("Connection closed {:?}!", connection.address);
    }
}
