use bytes::Bytes;
use tokio::net::TcpStream;

use super::tcp_server::ProtocolState;

pub trait PacketHandler {
    fn handle_packet(&mut self, stream: &mut TcpStream, state: &mut ProtocolState, bytes: Bytes, packet_id: i32);
}
