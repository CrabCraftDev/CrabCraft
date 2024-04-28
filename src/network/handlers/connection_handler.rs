use std::cmp::Ordering;
use std::fs;

use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bytes::{Bytes, BytesMut};
use crab_nbt::Nbt;
use log::info;
use serde_json::{json, Value};

use crab_protocol::buf::WriteByteBuf;
use crab_protocol::packets::configuration::{
    AcknowledgeFinishConfigurationC2SPacket, ClientInformationC2SPacket, FinishConfigurationS2CPacket,
    KeepAliveResponseC2SPacket, PluginMessageS2CPacket, RegistryDataS2CPacket,
};
use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket};
use crab_protocol::packets::play::{
    GameEventS2CPacket, GameJoinS2CPacket, SetDefaultSpawnPositionS2CPacket, SynchronizePlayerPositionS2CPacket,
    SynchronizePositionFlags,
};
use crab_protocol::packets::status::{
    PingRequestC2SPacket, PingResponseS2CPacket, StatusRequestC2SPacket, StatusResponseS2CPacket,
};
use crab_protocol::protocol::ProtocolState;

use crab_protocol::raw_packet::RawPacket;
use crab_protocol::types::block_position::BlockPosition;
use crab_protocol::types::game_mode::GameMode;
use crab_protocol::types::identifier::Identifier;
use crab_protocol::types::position::Position;

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::utils::errors::Error;
use crate::utils::fs::get_server_path;

pub struct ConnectionHandler;

const SERVER_PROTOCOL_VERSION: i32 = 765;
const SERVER_VERSION: &str = "1.20.4";

#[async_trait]
impl PacketHandler for ConnectionHandler {
    async fn handle_packet(&mut self, connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if connection.state == ProtocolState::Handshake {
            if let Some(packet) = raw_packet.decode::<HandshakeC2SPacket>() {
                connection.state = match packet.next_state {
                    NextState::Status => ProtocolState::Status,
                    NextState::Login => ProtocolState::Login,
                };

                if connection.state != ProtocolState::Login || packet.protocol_version == SERVER_PROTOCOL_VERSION {
                    return Ok(());
                }

                let message = match packet.protocol_version.cmp(&SERVER_PROTOCOL_VERSION) {
                    Ordering::Less => format!("Server is on newer Minecraft version! Use {}", SERVER_VERSION),
                    Ordering::Greater => format!("Server is on older Minecraft version! Use {}", SERVER_VERSION),
                    Ordering::Equal => "".to_string(),
                };
                connection.disconnect(&message).await;
                return Ok(());
            }
        }

        if connection.state == ProtocolState::Status {
            if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>() {
                connection
                    .send_packet(PingResponseS2CPacket {
                        payload: packet.payload,
                    })
                    .await;
            }

            if let Some(_packet) = raw_packet.decode::<StatusRequestC2SPacket>() {
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

                let json_response = serde_json::to_string(&status_response)?;
                connection.send_packet(StatusResponseS2CPacket { json_response }).await;

                info!("Pinged server list!");
            }
        }

        if connection.state == ProtocolState::Login {
            if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>() {
                connection
                    .send_packet(LoginSuccessS2CPacket {
                        uuid: packet.player_uuid,
                        username: packet.username,
                        properties: Vec::new(),
                    })
                    .await;
            }

            if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>() {
                connection.state = ProtocolState::Configuration;
                return Ok(());
            }
        }

        if connection.state == ProtocolState::Configuration {
            if let Some(_packet) = raw_packet.decode::<ClientInformationC2SPacket>() {
                let binary_registry_data: &[u8] = include_bytes!("../../../assets/registry-data.nbt");
                connection
                    .send_packet(RegistryDataS2CPacket {
                        registry_data: Nbt::read(&mut Bytes::from(binary_registry_data))?,
                    })
                    .await;

                let mut data = BytesMut::new();
                data.put_string("CrabCraft".to_string());

                connection
                    .send_packet(PluginMessageS2CPacket {
                        channel: Identifier::minecraft("brand")?,
                        data: data.freeze().into(),
                    })
                    .await;

                connection.send_packet(FinishConfigurationS2CPacket).await;
            }

            if let Some(_packet) = raw_packet.decode::<AcknowledgeFinishConfigurationC2SPacket>() {
                connection.state = ProtocolState::Play;

                connection
                    .send_packet(GameJoinS2CPacket {
                        entity_id: 0,
                        is_hardcore: false,
                        dimension_names: Vec::new(),
                        max_players: 100,
                        view_distance: 16,
                        simulation_distance: 20,
                        reduced_debug_info: false,
                        enable_respawn_screen: true,
                        do_limited_crafting: false,
                        dimension_type: Identifier::minecraft("overworld")?,
                        dimension_name: Identifier::minecraft("overworld")?,
                        hashed_seed: 99887766,
                        game_mode: GameMode::Creative,
                        previous_game_mode: None.into(),
                        is_debug: false,
                        is_flat: true,
                        last_death_location: None,
                        portal_cooldown: 20,
                    })
                    .await;

                connection
                    .send_packet(SetDefaultSpawnPositionS2CPacket {
                        location: BlockPosition { x: 0, y: 0, z: 0 },
                        angle: 0.0,
                    })
                    .await;

                connection
                    .send_packet(SynchronizePlayerPositionS2CPacket {
                        position: Position::new(0.0, 0.0, 0.0),
                        yaw: 0.0,
                        pitch: 0.0,
                        flags: SynchronizePositionFlags::new(),
                        teleport_id: 2137,
                    })
                    .await;

                connection
                    .send_packet(GameEventS2CPacket { event: 13, value: 0.0 })
                    .await;

                info!("Player joined!");
            }
        }

        if connection.state == ProtocolState::Play {
            if let Some(packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>() {
                if connection.keepalive_pending && connection.last_keepalive_time == packet.keep_alive_id {
                    connection.keepalive_pending = false;
                } else {
                    connection.disconnect("Timeout!").await;
                }
            }
        }

        Ok(())
    }
}
