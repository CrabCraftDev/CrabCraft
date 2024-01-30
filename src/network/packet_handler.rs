use bytes::Bytes;
use tokio::net::TcpStream;

pub trait PacketHandler {
    fn handle_packet(&mut self, stream: &mut TcpStream, bytes: Bytes, packet_id: i32);
}
