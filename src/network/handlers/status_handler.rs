use std::fs;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crab_protocol::packets::common::{PingRequestC2SPacket, PingResponseS2CPacket};
use log::info;

use crab_protocol::packets::status::{
    PlayersInfo, ServerInfo, ServerVersion, StatusRequestC2SPacket, StatusResponseS2CPacket,
};
use crab_protocol::raw_packet::RawPacket;

use crate::utils::errors::Error;
use crate::utils::fs::get_server_path;
use crate::{
    mc_server::{SERVER_PROTOCOL_VERSION, SERVER_VERSION},
    network::connection::Connection,
};

pub struct StatusHandler;

impl StatusHandler {
    pub async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        let state = *connection.state.read().unwrap();
        if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>(state) {
            connection.send_packet(PingResponseS2CPacket { id: packet.id }).await;
        }

        if let Some(_packet) = raw_packet.decode::<StatusRequestC2SPacket>(state) {
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
