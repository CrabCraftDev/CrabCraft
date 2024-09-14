use std::sync::{Arc, Mutex};

use chrono::Local;
use crab_nbt::NbtTag;
use crab_protocol::{
    buf::packet_from_stream,
    packet::Packet,
    packets::{
        common::{DisconnectS2CPacket, KeepAliveResponseC2SPacket, KeepAliveS2CPacket},
        login::LoginDisconnectS2CPacket,
    },
    protocol_state::ProtocolState,
    raw_packet::RawPacket,
};
use crossbeam::sync::ShardedLock;
use evenio::component::Component;
use kanal::{Receiver, Sender};
use log::{error, warn};
use serde_json::json;
use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

use crate::utils::errors::Error;

use super::{
    handlers::{handshake_handler, keep_alive_handler, login_handler, status_handler},
    keep_alive_state::KeepAliveState,
    packet_transformer::PacketTransformer,
};

const KEEPALIVE_INTERVAL_MS: i64 = 15_000;

/// Represents a network connection between player and the server.
/// Used to send packets, receive and manage connection
#[derive(Debug, Clone, Component)]
pub struct NetworkConnection {
    pub state: Arc<ShardedLock<ProtocolState>>,
    pub keep_alive_state: Arc<Mutex<KeepAliveState>>,
    pub transformer: PacketTransformer,
    pub incoming_connection_sender: Sender<IncomingConnection>,
    pub outgoing_packets_sender: Sender<RawPacket>,
    pub outgoing_packets_receiver: Receiver<RawPacket>,
    pub incoming_packets_sender: Sender<RawPacket>,
    pub incoming_packets_receiver: Receiver<RawPacket>,
}

impl NetworkConnection {
    pub fn new(incoming_connection_sender: Sender<IncomingConnection>) -> Self {
        let (outgoing_packets_sender, outgoing_packets_receiver) = kanal::unbounded::<RawPacket>();
        let (incoming_packets_sender, incoming_packets_receiver) = kanal::unbounded::<RawPacket>();

        NetworkConnection {
            state: Arc::new(ShardedLock::new(ProtocolState::Handshake)),
            keep_alive_state: Arc::new(Mutex::new(KeepAliveState::default())),
            transformer: PacketTransformer::new(),
            incoming_connection_sender,
            outgoing_packets_sender,
            outgoing_packets_receiver,
            incoming_packets_sender,
            incoming_packets_receiver,
        }
    }

    pub async fn accept(&self, stream: TcpStream) {
        let (read_half, write_half) = stream.into_split();

        let writer = tokio::spawn(NetworkConnection::writer(
            self.clone(),
            write_half,
            self.transformer.clone(),
        ));

        let reader = tokio::spawn(NetworkConnection::reader(
            self.clone(),
            read_half,
            self.transformer.clone(),
        ));

        let (writer_result, reader_result) = tokio::join!(writer, reader);
        if let Err(err) = writer_result {
            if err.is_panic() {
                error!("Player's connection writer task panicked {err:?}")
            }
        }
        if let Err(err) = reader_result {
            if err.is_panic() {
                error!("Player's connection reader task panicked {err:?}")
            }
        }
    }

    async fn writer(connection: NetworkConnection, mut read_half: OwnedWriteHalf, transformer: PacketTransformer) {
        let receiver = connection.outgoing_packets_receiver.clone().to_async();
        loop {
            let Ok(packet) = receiver.recv().await else {
                break;
            };
            let bytes = match transformer.encode(packet) {
                Ok(bytes) => bytes,
                Err(err) => {
                    error!("Serialization error: {err}");
                    connection.close();
                    break;
                }
            };
            if read_half.write_all(&bytes).await.is_err() {
                connection.close();
                break;
            }
            connection.keep_connection_alive();
        }
    }

    async fn reader(connection: NetworkConnection, mut read_half: OwnedReadHalf, mut transformer: PacketTransformer) {
        loop {
            if connection.is_closed() {
                break;
            }
            let packet = match NetworkConnection::next_packet(&mut read_half, &mut transformer).await {
                Ok(Some(packet)) => packet,
                Ok(None) => {
                    connection.close();
                    break;
                }
                Err(err) => {
                    warn!("Error decoding packet {err:?}!");
                    connection.disconnect("Invalid packet received!".to_owned());
                    break;
                }
            };
            connection.process_packet(packet).await;
        }
    }

    async fn next_packet(
        read_half: &mut OwnedReadHalf,
        transformer: &mut PacketTransformer,
    ) -> Result<Option<RawPacket>, Error> {
        Ok(packet_from_stream(read_half)
            .await?
            .map(|buffer| transformer.decode(buffer))
            .transpose()?)
    }

    pub async fn process_packet(&self, raw_packet: RawPacket) {
        let state = self.get_state();
        match state {
            ProtocolState::Status => status_handler::handle_packet(self, raw_packet),
            ProtocolState::Handshake => handshake_handler::handle_packet(self, raw_packet),
            ProtocolState::Login => login_handler::handle_packet(self, raw_packet),
            ProtocolState::Configuration | ProtocolState::Play => {
                if let Some(keep_alive_packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>(state) {
                    keep_alive_handler::handle_packet(self, keep_alive_packet);
                } else if self.incoming_packets_sender.send(raw_packet).is_err() {
                    warn!("Tried to transfer received packet to main while channel already closed")
                }
            }
        };
    }

    pub fn send_packet<P: Packet>(&self, packet: P) {
        match RawPacket::try_from_packet(packet, self.get_state()) {
            Ok(raw_packet) => {
                let _ = self
                    .outgoing_packets_sender
                    .send(raw_packet)
                    .inspect_err(|_| warn!("Tried to send packet while connection closed"));
            }
            Err(err) => {
                error!("Serialization error: {err}");
            }
        };
    }

    pub fn disconnect(&self, reason: String) {
        if self.is_closed() {
            warn!("Called disconnect() on a closed connection");
            return;
        }
        match self.get_state() {
            ProtocolState::Configuration | ProtocolState::Play => {
                self.send_packet(DisconnectS2CPacket {
                    reason: NbtTag::String(reason.clone()).into(),
                });
            }
            ProtocolState::Login => {
                self.send_packet(LoginDisconnectS2CPacket {
                    reason: json!({"text": reason}).to_string().into(),
                });
            }
            _ => {}
        }
        self.close();
    }

    pub fn is_closed(&self) -> bool {
        self.incoming_packets_sender.is_closed() || self.outgoing_packets_sender.is_closed()
    }

    fn close(&self) {
        self.incoming_packets_sender.close();
        self.outgoing_packets_sender.close();
    }

    pub fn get_state(&self) -> ProtocolState {
        *self.state.read().unwrap()
    }

    pub fn set_state(&self, new_state: ProtocolState) {
        *self.state.write().unwrap() = new_state;
    }

    pub fn keep_connection_alive(&self) {
        let state = self.get_state();
        if state != ProtocolState::Configuration && state != ProtocolState::Play {
            return;
        }

        let Ok(mut keep_alive_state) = self.keep_alive_state.lock() else {
            return;
        };

        let current_time = Local::now().timestamp_millis();
        if current_time - keep_alive_state.last_keep_alive_time < KEEPALIVE_INTERVAL_MS {
            return;
        }

        if keep_alive_state.keep_alive_pending {
            self.disconnect("Timeout!".to_owned());
            return;
        }

        keep_alive_state.keep_alive_pending = true;
        keep_alive_state.last_keep_alive_time = current_time;

        self.send_packet(KeepAliveS2CPacket {
            keep_alive_id: current_time,
        });
    }
}
