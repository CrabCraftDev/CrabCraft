use bytes::Bytes;

use crate::network::tcp_server::Connection;

pub trait PacketHandler {
    fn handle_packet(&mut self, connection_data: &mut Connection, bytes: Bytes, packet_id: i32);
}
