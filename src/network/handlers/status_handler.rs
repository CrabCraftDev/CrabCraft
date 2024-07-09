use std::fs;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crab_protocol::packets::common::{PingRequestC2SPacket, PingResponseS2CPacket};

use crab_protocol::packets::status::{
    PlayersInfo, ServerInfo, ServerVersion, StatusRequestC2SPacket, StatusResponseS2CPacket,
};
use crab_protocol::raw_packet::RawPacket;
use log::warn;

use crate::utils::fs::get_server_path;
use crate::{
    mc_server::{SERVER_PROTOCOL_VERSION, SERVER_VERSION},
    network::connection::NetworkConnection,
};

const BASE64_ENCODING_PREFIX: &str = "data:image/png;base64,";
const SERVER_ICON_PATH: &str = "server-icon.png";

pub fn handle_packet(connection: &NetworkConnection, raw_packet: RawPacket) {
    let state = connection.get_state();
    if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>(state) {
        connection.send_packet(PingResponseS2CPacket { id: packet.id });
        connection.disconnect("".to_owned());
    }

    if raw_packet.decode::<StatusRequestC2SPacket>(state).is_some() {
        handle_status_request(connection);
    }
}

fn handle_status_request(connection: &NetworkConnection) {
    let server_icon_path = get_server_path().join(SERVER_ICON_PATH);
    let mut favicon = String::new();
    if server_icon_path.exists() {
        match fs::read(server_icon_path) {
            Ok(png_data) => {
                favicon = format!("{BASE64_ENCODING_PREFIX}{}", BASE64_STANDARD.encode(png_data));
            }
            Err(err) => {
                warn!("Couldn't read server icon! {err}");
            }
        };
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

    connection.send_packet(StatusResponseS2CPacket { server_info });
}
