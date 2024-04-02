use std::cmp::Ordering;
use std::fs;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use log::info;
use serde_json::{json, Value};

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::network::tcp_server::{ProtocolState, TcpServer};
use crate::utils::fs::get_server_path;

pub struct ConnectionHandler;

const SERVER_PROTOCOL_VERSION: i32 = 765;
const SERVER_VERSION: &str = "1.20.4";

#[async_trait]
impl PacketHandler for ConnectionHandler {
    fn handle_packet(&mut self, connection_data: &mut Connection, mut bytes: Bytes, packet_id: i32) {
        // =================== HANDSHAKE
        if packet_id == 0x00 && connection_data.state == ProtocolState::Handshake {
            let version = bytes.get_var_int().unwrap();
            let _address_ = bytes.get_string();
            let _port = bytes.get_u16();
            let next_state = bytes.get_var_int().unwrap();

            connection_data.state = match next_state {
                1 => ProtocolState::Status,
                2 => ProtocolState::Login,
                _ => panic!("Invalid state!"),
            };

            if connection_data.state == ProtocolState::Login && version != SERVER_PROTOCOL_VERSION {
                match version.cmp(&SERVER_PROTOCOL_VERSION) {
                    Ordering::Less => connection_data
                        .disconnect_sync(&format!("Server is on newer Minecraft version! Use {SERVER_VERSION}")),
                    Ordering::Greater => connection_data
                        .disconnect_sync(&format!("Server is on older Minecraft version! Use {SERVER_VERSION}")),
                    Ordering::Equal => {}
                }
            }

            return;
        }

        // =================== STATUS
        // Ping request
        if packet_id == 0x01 && connection_data.state == ProtocolState::Status {
            let time = bytes.get_u64();
            let mut response = BytesMut::new();
            response.put_var_int(0x01);
            response.put_u64(time);
            TcpServer::send(connection_data, x).await;
            return;
        }

        // Server list ping
        if packet_id == 0x00 && connection_data.state == ProtocolState::Status {
            let mut response = BytesMut::new();
            response.put_var_int(0);

            let mut status_response = json!({
                "version": {
                    "name": SERVER_VERSION,
                    "protocol": SERVER_PROTOCOL_VERSION,
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

            let server_icon_path = get_server_path().join("server-icon.png");
            if server_icon_path.exists() {
                let png_data = fs::read(server_icon_path).expect("Couldn't read server icon!");
                let mut buf = "data:image/png;base64,".to_owned();
                let encoded_png = BASE64_STANDARD.encode(png_data);
                buf.push_str(&encoded_png);
                status_response["favicon"] = Value::String(buf);
            }

            response
                .put_string(serde_json::to_string(&status_response).unwrap())
                .unwrap();

            TcpServer::send(connection_data, response.freeze()).await;

            info!("Pinged server list!");
            return;
        }

        // =================== LOGIN
        // Login start
        if packet_id == 0x00 && connection_data.state == ProtocolState::Login {
            let username = bytes.get_string().unwrap();
            let uuid = bytes.get_uuid();

            let mut response = BytesMut::new();
            response.put_var_int(0x02);
            response.put_uuid(uuid);
            response.put_string(username).unwrap();
            response.put_var_int(0);

            TcpServer::send(connection_data, response.freeze()).await;
        }

        // Login acknowledged
        if packet_id == 0x03 && connection_data.state == ProtocolState::Login {
            connection_data.state = ProtocolState::Configuration;
            return;
        }

        // =================== CONFIGURATION
        if packet_id == 0x00 && connection_data.state == ProtocolState::Configuration {
            let _language = bytes.get_string();
            let _view_distance = bytes.get_byte();
            let _chat_mode = bytes.get_var_int();
            let _chat_colors = bytes.get_boolean();
            let _displayed_skin_parts = bytes.get_unsigned_byte();
            let _main_hand = bytes.get_var_int();
            let _text_filtering = bytes.get_boolean();
            let _allow_server_listening = bytes.get_boolean();

            let packet = include_bytes!("../../../assets/registry-data.nbt");
            TcpServer::send(connection_data, Bytes::from(packet.to_vec())).await;

            // Server brand packet
            let mut response = BytesMut::new();
            response.put_var_int(0x0);
            response.put_string("minecraft:brand".to_string()).unwrap();
            response.put_string("CrabCraft".to_string()).unwrap();
            TcpServer::send(connection_data, response.freeze()).await;

            let mut response = BytesMut::new();
            response.put_var_int(2);
            TcpServer::send(connection_data, response.freeze()).await;
        }

        // Plugin Message
        if packet_id == 0x01 && connection_data.state == ProtocolState::Configuration {
            let _namespace = bytes.get_string();
            let _data = Bytes::from(bytes.get_bytes(bytes.len() as i32));
        }

        // Finish Configuration
        if packet_id == 0x02 && connection_data.state == ProtocolState::Configuration {
            connection_data.state = ProtocolState::Play;
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
            response.put_string("minecraft:overworld".to_string()).unwrap(); // Dimension Type
            response.put_string("overworld".to_string()).unwrap(); // Dimension Name
            response.put_u64(99887766); // Hashed Seed
            response.put_unsigned_byte(1); // Gamemode (1 - creative)
            response.put_byte(-1); // Previous game mode
            response.put_boolean(false); // Is Debug
            response.put_boolean(true); // Is Flat
            response.put_boolean(false); // Has Death Location
            response.put_var_int(20); // Portal Cooldown

            TcpServer::send(connection_data, response.freeze()).await;

            // Set Default Spawn Position
            let mut response = BytesMut::new();
            response.put_var_int(0x54);
            response.put_i64(0b0100011000000111011000110010110000010101101101001000001100111111);
            response.put_f32(0.0);
            TcpServer::send(connection_data, response.freeze()).await;

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
            TcpServer::send(connection_data, response.freeze()).await;

            // Game Event - Start waiting for level chunks
            let mut response = BytesMut::new();
            response.put_var_int(0x20);
            response.put_unsigned_byte(13);
            response.put_f32(0.0);
            TcpServer::send_sync(connection_data, response.freeze());

            info!("Player joined!");
        }

        // =================== PLAY
        if packet_id == 0x15 && connection_data.state == ProtocolState::Play {
            let keep_alive_challenge = bytes.get_u64() as i64;

            if connection_data.keepalive_pending && connection_data.last_keepalive_time == keep_alive_challenge {
                connection_data.keepalive_pending = false
            } else {
                connection_data.disconnect_sync("Timeout!");
            }
        }
    }
}
