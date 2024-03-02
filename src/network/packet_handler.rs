use bytes::Bytes;

use crate::network::connection::Connection;

pub trait PacketHandler {
    fn handle_packet(&mut self, connection_data: &mut Connection, bytes: Bytes, packet_id: i32);
}
