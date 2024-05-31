use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use chrono::Local;
use crab_nbt::NbtTag;
use crossbeam::sync::ShardedLock;
use log::{error, info, warn};
use serde_json::json;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;

use crab_protocol::packet::Packet;
use crab_protocol::packets::configuration::DisconnectS2CPacket as ConfigurationDisconnectPacket;
use crab_protocol::packets::configuration::KeepAliveS2CPacket as ConfigurationKeepAlivePacket;
use crab_protocol::packets::login::DisconnectS2CPacket as LoginDisconnectPacket;
use crab_protocol::packets::play::DisconnectS2CPacket as PlayDisconnectPacket;
use crab_protocol::packets::play::KeepAliveS2CPacket as PlayKeepAlivePacket;
use crab_protocol::protocol::ProtocolState;
use crab_protocol::types::text::TextComponent;
use crab_protocol::{buf::to_var_int, raw_packet::RawPacket};

use crate::utils::errors::Error;

#[derive(Debug)]
pub struct Connection {
    pub(crate) state: Arc<ShardedLock<ProtocolState>>,
    pub(crate) write_half: WriteHalf<TcpStream>,
    pub(crate) address: SocketAddr,
    pub(crate) is_closed: bool,
    pub(crate) keepalive_pending: bool,
    pub(crate) last_keepalive_time: i64,
}

const KEEPALIVE_INTERVAL_MS: i64 = 15_000;

impl Connection {
    async fn do_send(&mut self, bytes: Bytes) -> Result<(), Error> {
        let length = bytes.len() as i32;
        self.write_half.write_all(&to_var_int(length)).await?;
        self.write_half.write_all(&bytes).await?;
        Ok(())
    }

    pub async fn send_packet(&mut self, packet: impl Packet) {
        match packet.serialize_packet() {
            Ok(bytes) => self.send(bytes).await,
            Err(err) => {
                error!("Serialization error: {}", err);
                self.close().await;
            }
        }
    }

    pub async fn send_raw_packet(&mut self, packet: RawPacket) {
        self.send(packet.to_bytes()).await;
    }

    pub async fn send(&mut self, bytes: Bytes) {
        if self.is_closed {
            return;
        }

        let send_result = self.do_send(bytes).await;
        if send_result.is_err() {
            self.close().await;
        }
    }

    pub async fn disconnect(&mut self, reason: &str) {
        if self.is_closed {
            return;
        }

        let reason = reason.to_string();
        let reason_text_component: TextComponent = NbtTag::String(reason.clone()).into();

        let state = *self.state.read().unwrap();
        match state {
            ProtocolState::Configuration => {
                self.send_packet(ConfigurationDisconnectPacket {
                    reason: reason_text_component,
                })
                .await;
            }
            ProtocolState::Play => {
                self.send_packet(PlayDisconnectPacket {
                    reason: reason_text_component,
                })
                .await;
            }
            ProtocolState::Login => {
                self.send_packet(LoginDisconnectPacket {
                    reason: json!({"text": reason}).to_string().into(),
                })
                .await;
            }
            _ => {}
        }
        self.close().await;
    }

    async fn close(&mut self) {
        let shutdown_result = self.write_half.shutdown().await;
        if let Err(err) = shutdown_result {
            warn!("{}", err.to_string());
        }
        self.is_closed = true;
        info!("Disconnected player!");
    }

    pub async fn keep_connection_alive(&mut self) {
        let state = *self.state.read().unwrap();
        if state != ProtocolState::Configuration && state != ProtocolState::Play {
            return;
        }

        let current_time = Local::now().timestamp_millis();
        if current_time - self.last_keepalive_time < KEEPALIVE_INTERVAL_MS {
            return;
        }

        if self.keepalive_pending {
            self.disconnect("Timeout!").await;
            return;
        }

        self.keepalive_pending = true;
        self.last_keepalive_time = current_time;

        let keep_alive_id = current_time;
        if state == ProtocolState::Play {
            self.send_packet(PlayKeepAlivePacket { keep_alive_id }).await;
        } else {
            self.send_packet(ConfigurationKeepAlivePacket { keep_alive_id }).await;
        };
    }
}
