use crate::network::tcp_server::ConnectionData;
use bytes::Bytes;
use tokio::net::TcpStream;

pub trait PacketHandler {
    fn handle_packet(&mut self, stream: &mut TcpStream, data: &mut ConnectionData, bytes: Bytes, packet_id: i32);
}
