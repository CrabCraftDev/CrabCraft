use std::sync::{Arc, Mutex};

use chrono::Local;
use crab_nbt::NbtTag;
use crab_protocol::{
    packet::Packet,
    packets::{
        common::{DisconnectS2CPacket, KeepAliveS2CPacket},
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
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::network::packet_receiver::PacketReceiver;

use super::{keepalive_state::KeepAliveState, packet_transformer::PacketTransformer};

const KEEPALIVE_INTERVAL_MS: i64 = 15_000;

#[derive(Debug, Component, Clone)]
pub struct NetworkConnection {
    pub state: Arc<ShardedLock<ProtocolState>>,
    pub keep_alive_state: Arc<Mutex<KeepAliveState>>,
    pub transformer: PacketTransformer,
    pub new_player_connection_sender: Sender<NetworkConnection>,
    pub outgoing_packets_sender: Sender<RawPacket>,
    pub outgoing_packets_receiver: Receiver<RawPacket>,
    pub incoming_packets_sender: Sender<RawPacket>,
    pub incoming_packets_receiver: Receiver<RawPacket>,
}

impl NetworkConnection {
    pub fn new(new_player_connection_sender: Sender<NetworkConnection>) -> Self {
        let (outgoing_packets_sender, outgoing_packets_receiver) = kanal::unbounded::<RawPacket>();
        let (incoming_packets_sender, incoming_packets_receiver) = kanal::unbounded::<RawPacket>();

        NetworkConnection {
            state: Arc::new(ShardedLock::new(ProtocolState::Handshake)),
            keep_alive_state: Arc::new(Mutex::new(KeepAliveState::new())),
            transformer: PacketTransformer::new(),
            new_player_connection_sender,
            outgoing_packets_sender,
            outgoing_packets_receiver,
            incoming_packets_sender,
            incoming_packets_receiver,
        }
    }

    pub async fn accept(&self, stream: TcpStream) {
        let (read_half, mut write_half) = stream.into_split();
        let mut receiver = PacketReceiver::new(read_half, self.transformer.clone(), self.clone());

        let outgoing_packets_receiver = self.outgoing_packets_receiver.clone();

        let writer_transformer = self.transformer.clone();
        let writer_connection = self.clone();

        let writer = tokio::spawn(async move {
            let receiver = outgoing_packets_receiver.to_async();
            while let Ok(packet) = receiver.recv().await {
                let bytes = match writer_transformer.encode(packet) {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        error!("Serialization error: {}", err);
                        writer_connection.close();
                        break;
                    }
                };
                if write_half.write_all(&bytes).await.is_err() {
                    writer_connection.close();
                }
            }
        });

        let reader = tokio::spawn(async move {
            loop {
                let packets = match receiver.receive_packets().await {
                    Ok(packets) => packets,
                    Err(err) => {
                        warn!("Error handling packet {:?}!", err);
                        break;
                    }
                };
                let Some(packets) = packets else {
                    break;
                };

                for packet in packets {
                    receiver.process_packet(packet).await;
                }
            }
            receiver.connection.close()
        });

        tokio::select! {_ = writer => () , _ = reader => ()};
    }

    pub fn send_packet<P: Packet>(&self, packet: P) {
        match RawPacket::try_from_packet(packet, self.get_state()) {
            Ok(raw_packet) => self
                .outgoing_packets_sender
                .send(raw_packet)
                .expect("Tried to send packet while connection closed"),
            Err(err) => {
                error!("Serialization error: {}", err);
            }
        };
    }

    pub fn disconnect(&self, reason: String) {
        let state = self.get_state();
        match state {
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
        if current_time - keep_alive_state.last_keepalive_time < KEEPALIVE_INTERVAL_MS {
            return;
        }

        if keep_alive_state.keep_alive_pending {
            self.disconnect("Timeout!".to_owned());
            return;
        }

        keep_alive_state.keep_alive_pending = true;
        keep_alive_state.last_keepalive_time = current_time;

        self.send_packet(KeepAliveS2CPacket {
            keep_alive_id: current_time,
        });
    }
}
