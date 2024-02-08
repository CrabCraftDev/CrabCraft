use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde_json::json;
use tokio::net::TcpStream;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::network::packet_byte_buf::read_byte_buf::ReadByteBuf;
use crate::network::packet_byte_buf::write_byte_buf::WriteByteBuf;
use crate::network::packet_handler::PacketHandler;
use crate::network::tcp_server::{ConnectionData, ProtocolState, TcpServer};

pub struct ConnectionHandler;

impl PacketHandler for ConnectionHandler {
    fn handle_packet(&mut self, stream: &mut TcpStream, data: &mut ConnectionData, mut bytes: Bytes, packet_id: i32) {
        // =================== HANDSHAKE
        if packet_id == 0x00 && data.state == ProtocolState::Handshake {
            let _version = bytes.get_var_int();
            let _address_ = bytes.get_string();
            let _port = bytes.get_u16();
            let next_state = bytes.get_var_int();

            data.state = match next_state {
                1 => ProtocolState::Status,
                2 => ProtocolState::Login,
                _ => panic!("Invalid state!"),
            };

            return;
        }

        // =================== STATUS
        // Ping request
        if packet_id == 0x01 && data.state == ProtocolState::Status {
            let time = bytes.get_u64();
            let mut response = BytesMut::new();
            response.put_var_int(0x01);
            response.put_u64(time);
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
            return;
        }

        // Server list ping
        if packet_id == 0x00 && data.state == ProtocolState::Status {
            let mut response = BytesMut::new();
            response.put_var_int(0);

            let status_response = json!({
                "version": {
                    "name": "1.20.4",
                    "protocol": 765,
                },
                "players": {
                    "online": 0,
                    "max": 100,
                    "sample": Vec::<String>::new(),
                },
                "description": "CrabCraft!",
                "enforces_secure_chat": false,
                "previews_chat": true,
            });

            response.put_string(serde_json::to_string(&status_response).unwrap());

            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
            return;
        }

        // =================== LOGIN
        // Login start
        if packet_id == 0x00 && data.state == ProtocolState::Login {
            let username = bytes.get_string();
            let uuid = bytes.get_uuid();

            let mut response = BytesMut::new();
            response.put_var_int(0x02);
            response.put_uuid(uuid);
            response.put_string(username);
            response.put_var_int(0);

            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
        }

        // Login acknowledged
        if packet_id == 0x03 && data.state == ProtocolState::Login {
            data.state = ProtocolState::Configuration;
            return;
        }

        // =================== CONFIGURATION
        if packet_id == 0x00 && data.state == ProtocolState::Configuration {
            let _language = bytes.get_string();
            let _view_distance = bytes.get_byte();
            let _chat_mode = bytes.get_var_int();
            let _chat_colors = bytes.get_boolean();
            let _displayed_skin_parts = bytes.get_unsigned_byte();
            let _main_hand = bytes.get_var_int();
            let _text_filtering = bytes.get_boolean();
            let _allow_server_listening = bytes.get_boolean();

            let packet = include_bytes!("../../../assets/registry-data.nbt");
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, Bytes::from(packet.to_vec()))));

            // Server brand packet
            let mut response = BytesMut::new();
            response.put_var_int(0x0);
            response.put_string("minecraft:brand".to_string());
            response.put_string("CrabCraft".to_string());
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));

            let mut response = BytesMut::new();
            response.put_var_int(2);
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
        }

        // Plugin Message
        if packet_id == 0x01 && data.state == ProtocolState::Configuration {
            let _namespace = bytes.get_string();
            let _data = Bytes::from(bytes.get_bytes(bytes.len() as i32));
        }

        // Finish Configuration
        if packet_id == 0x02 && data.state == ProtocolState::Configuration {
            data.state = ProtocolState::Play;
            let mut response = BytesMut::new();
            response.put_var_int(0x29);
            response.put_u32(0); // Entity ID
            response.put_boolean(false); // Is hardcore
            response.put_var_int(0); // Dimension Count
            response.put_var_int(100); // Max players
            response.put_var_int(16); // View distance
            response.put_var_int(20); // Simulation distance
            response.put_boolean(false); // Reduced debug info
            response.put_boolean(true); // Enable respawn screen
            response.put_boolean(true); // Do limited crafting
            response.put_string("minecraft:overworld".to_string()); // Dimension Type
            response.put_string("overworld".to_string()); // Dimension Name
            response.put_u64(99887766); // Hashed Seed
            response.put_unsigned_byte(1); // Gamemode (1 - creative)
            response.put_byte(-1); // Previous game mode
            response.put_boolean(true); // Is Debug
            response.put_boolean(true); // Is Flat
            response.put_boolean(false); // Has Death Location
            response.put_var_int(20); // Portal Cooldown

            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));

            // Set Default Spawn Position
            let mut response = BytesMut::new();
            response.put_var_int(0x54);
            response.put_i64(0b0100011000000111011000110010110000010101101101001000001100111111);
            response.put_f32(0.0);
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));

            // Synchronize Player Position
            let mut response = BytesMut::new();
            response.put_var_int(0x3E);
            response.put_f64(0.0); // Position
            response.put_f64(0.0);
            response.put_f64(0.0);
            response.put_f32(0.0); // Rotation
            response.put_f32(0.0);
            response.put_byte(0); // Flags
            response.put_var_int(2137); // Teleport ID
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));

            // Game Event - Start waiting for level chunks
            let mut response = BytesMut::new();
            response.put_var_int(0x20);
            response.put_unsigned_byte(13);
            response.put_f32(0.0);
            block_in_place(|| Handle::current().block_on(TcpServer::send(stream, response.freeze())));
        }
    }
}
