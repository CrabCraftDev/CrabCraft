use std::fs;

use bytes::{Buf, Bytes, BytesMut};
use log::debug;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::network::packet_byte_buffer::{ReadByteBuf, WriteByteBuf};
use crate::network::packet_handler::PacketHandler;
use crate::network::tcp_server::TcpServer;

#[derive(Debug)]
pub struct HandshakeHandler;

impl PacketHandler for HandshakeHandler {
    fn handle_packet(&self, stream: &mut TcpStream, mut bytes: Bytes, packet_id: i32) {
        if packet_id != 0 {
            return;
        }

        let _version = bytes.get_var_int();
        let _address_ = bytes.get_string();
        let _port = bytes.get_u16();
        let status = bytes.get_var_int();
        debug!("Handshake packet! Status: {}", status);

        if status != 1 {
            return;
        }

        let mut new_response = BytesMut::new();
        new_response.put_var_int(0);
        let status = fs::read_to_string("logs/status.json").expect("Couldn't find logs/status.json");
        new_response.put_string(status.replace(' ', ""));

        let mut len_bytess = BytesMut::new();
        len_bytess.put_var_int(new_response.len() as i32);

        let mut response = Vec::new();
        response.extend_from_slice(&len_bytess);
        response.extend_from_slice(&new_response);

        block_in_place(|| {
            Handle::current().block_on(TcpServer::send(stream, response));
        });
    }
}
