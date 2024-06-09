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
use crab_protocol::packets::common::{DisconnectS2CPacket, KeepAliveS2CPacket};
use crab_protocol::packets::login::LoginDisconnectS2CPacket;
use crab_protocol::protocol::ProtocolState;
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
        let state = *self.state.read().unwrap();
        match packet.serialize_packet(state) {
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

        let state = *self.state.read().unwrap();
        match state {
            ProtocolState::Configuration | ProtocolState::Play => {
                self.send_packet(DisconnectS2CPacket {
                    reason: NbtTag::String(reason.clone()).into(),
                })
                .await;
            }
            ProtocolState::Login => {
                self.send_packet(LoginDisconnectS2CPacket {
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

        self.send_packet(KeepAliveS2CPacket {
            keep_alive_id: current_time,
        })
        .await;
    }
}
