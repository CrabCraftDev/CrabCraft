use std::fs;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::info;

use crab_protocol::packets::status::{
    PingRequestC2SPacket, PingResponseS2CPacket, PlayersInfo, ServerInfo, ServerVersion, StatusRequestC2SPacket,
    StatusResponseS2CPacket,
};
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::utils::errors::Error;
use crate::utils::fs::get_server_path;
use crate::{SERVER_PROTOCOL_VERSION, SERVER_VERSION};

pub struct StatusHandler;

impl StatusHandler {
    pub async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>() {
            connection
                .send_packet(PingResponseS2CPacket {
                    payload: packet.payload,
                })
                .await;
        }

        if let Some(_packet) = raw_packet.decode::<StatusRequestC2SPacket>() {
            let server_icon_path = get_server_path().join("server-icon.png");
            let mut favicon = String::new();
            if server_icon_path.exists() {
                let png_data = fs::read(server_icon_path).expect("Couldn't read server icon!");
                let encoded_png = BASE64_STANDARD.encode(png_data);
                favicon = "data:image/png;base64,".to_owned() + &encoded_png;
            }

            let server_info = ServerInfo {
                version: ServerVersion {
                    name: SERVER_VERSION.to_string(),
                    protocol_version: SERVER_PROTOCOL_VERSION,
                },
                players: PlayersInfo {
                    max: 100,
                    online: 0,
                    sample: Vec::new(),
                },
                description: "CrabCraft!".to_string(),
                favicon,
                enforces_secure_chat: false,
                previews_chat: false,
            };

            connection.send_packet(StatusResponseS2CPacket { server_info }).await;

            info!("Pinged server list!");
        }

        Ok(())
    }
}
