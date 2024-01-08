use bytes::{Buf, Bytes, BytesMut};
use log::debug;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::network::packet_byte_buf::write_byte_buf::WriteByteBuf;
use crate::network::packet_handler::PacketHandler;
use crate::network::tcp_server::TcpServer;
use crate::types::server_status_response::{Description, Players, ServerStatusResponse, Version};

pub struct HandshakeHandler;

impl PacketHandler for HandshakeHandler {
    fn handle_packet(&self, stream: &mut TcpStream, mut bytes: Bytes, packet_id: i32) {
        if packet_id != 0 {
            return;
        }

        // Status Request packet
        if bytes.is_empty() {
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

        let mut response = BytesMut::new();
        response.put_var_int(0);
        let status_response = ServerStatusResponse {
            version: Version {
                name: String::from("1.20.2"),
                protocol: 764,
            },
            players: Players {
                max: 100,
                online: 0,
                sample: Vec::new(),
            },
            description: Description {
                text: String::from("CrabCraft!"),
            },
            enforces_secure_chat: true,
            previews_chat: true,
        };
        response.put_string(serde_json::to_string(&status_response).unwrap());

        block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
    }
}
