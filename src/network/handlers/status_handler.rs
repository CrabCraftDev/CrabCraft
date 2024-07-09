use std::fs;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crab_protocol::packets::common::{PingRequestC2SPacket, PingResponseS2CPacket};
use log::info;

use crab_protocol::packets::status::{
    PlayersInfo, ServerInfo, ServerVersion, StatusRequestC2SPacket, StatusResponseS2CPacket,
};
use crab_protocol::raw_packet::RawPacket;

use crate::utils::fs::get_server_path;
use crate::{
    mc_server::{SERVER_PROTOCOL_VERSION, SERVER_VERSION},
    network::connection::NetworkConnection,
};

pub struct StatusHandler {
    connection: NetworkConnection,
}

impl StatusHandler {
    pub fn new(connection: NetworkConnection) -> Self {
        Self { connection }
    }

    pub fn handle_packet(&self, raw_packet: RawPacket) {
        let state = self.connection.get_state();
        if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>(state) {
            self.connection.send_packet(PingResponseS2CPacket { id: packet.id });
            self.connection.disconnect("".to_owned());
        }

        if raw_packet.decode::<StatusRequestC2SPacket>(state).is_some() {
            self.handle_status_request();
        }
    }

    pub fn handle_status_request(&self) {
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

        self.connection.send_packet(StatusResponseS2CPacket { server_info });

        info!("Pinged server list!");
    }
}
