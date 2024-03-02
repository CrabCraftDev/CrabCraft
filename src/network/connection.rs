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

impl Connection {
    pub async fn disconnect(&mut self, _message: &str) {
        info!("Disconnected player!");

        // ToDo: Send disconnect packet. For now we don't have NBT support

        self.write_half.shutdown().await.unwrap();
    }

    pub async fn keep_connection_alive(&mut self) {
        let current_time = Local::now().timestamp_millis();
        if current_time - self.last_keepalive_time >= 15000 {
            if self.keepalive_pending {
                self.disconnect("Timeout!").await;
            } else {
                self.keepalive_pending = true;
                self.last_keepalive_time = current_time;

                let mut response = BytesMut::new();
                response.put_var_int(0x24);
                response.put_u64(current_time as u64);
                TcpServer::send(&mut self.write_half, response.freeze()).await;
            }
        }
    }
}
