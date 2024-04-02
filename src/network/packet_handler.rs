use async_trait::async_trait;
use bytes::Bytes;


use crate::network::connection::Connection;

#[async_trait]
pub trait PacketHandler {
    async fn handle_packet(&mut self, connection_data: &mut Connection, bytes: Bytes, packet_id: i32);
}
