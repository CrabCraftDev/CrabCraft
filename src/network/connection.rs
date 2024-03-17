use std::net::SocketAddr;

use bytes::{BufMut, BytesMut};
use chrono::Local;
use log::{info, warn};
use serde_json::json;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::network::packet_byte_buf::write_byte_buf::WriteByteBuf;
use crate::network::tcp_server::{ProtocolState, TcpServer};

#[derive(Debug)]
pub struct Connection {
    pub(crate) state: ProtocolState,
    pub(crate) write_half: WriteHalf<TcpStream>,
    pub(crate) address: SocketAddr,
    pub(crate) is_closed: bool,
    pub(crate) keepalive_pending: bool,
    pub(crate) last_keepalive_time: i64,
}

const KEEPALIVE_INTERVAL_MS: i64 = 15_000;

impl Connection {
    pub async fn disconnect(&mut self, message: &str) {
        let payload = message.as_bytes();
        let mut response = BytesMut::new();

        match self.state {
            ProtocolState::Configuration | ProtocolState::Play => {
                let packet_type = if self.state == ProtocolState::Play { 0x1B } else { 0x01 };
                response.put_var_int(packet_type);
                // NBT representation of TAG_String
                response.put_u8(8);
                response.put_u16(payload.len() as u16);
                response.put(payload);
            }
            ProtocolState::Login => {
                response.put_var_int(0x00);
                response.put_string(json!({"text": message}).to_string());
            }
            _ => {}
        }

        if !response.is_empty() {
            TcpServer::send(self, response.freeze()).await;
        }
        let sresult = self.write_half.shutdown().await;
        if sresult.is_err() {
            warn!("{}", sresult.err().unwrap().to_string());
        }
        info!("Disconnected player!");
    }

    pub fn disconnect_sync(&mut self, message: &str) {
        block_in_place(|| Handle::current().block_on(self.disconnect(message)));
    }

    pub async fn keep_connection_alive(&mut self) {
        if self.state != ProtocolState::Configuration && self.state != ProtocolState::Play {
            return;
        }

        let current_time = Local::now().timestamp_millis();
        if current_time - self.last_keepalive_time >= KEEPALIVE_INTERVAL_MS {
            if self.keepalive_pending {
                self.disconnect("Timeout!").await;
            } else {
                self.keepalive_pending = true;
                self.last_keepalive_time = current_time;

                let packet_id = if self.state == ProtocolState::Play { 0x24 } else { 0x03 };
                let mut response = BytesMut::new();
                response.put_var_int(packet_id);
                response.put_u64(current_time as u64);
                TcpServer::send(self, response.freeze()).await;
            }
        }
    }
}
