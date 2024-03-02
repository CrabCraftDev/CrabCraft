use std::net::SocketAddr;

use bytes::{BufMut, BytesMut};
use chrono::Local;
use log::info;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;

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
    pub async fn disconnect(&mut self, _message: &str) {
        info!("Disconnected player!");

        // ToDo: Send disconnect packet. For now we don't have NBT support

        self.write_half.shutdown().await.unwrap();
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
                TcpServer::send(&mut self.write_half, response.freeze()).await;
            }
        }
    }
}
