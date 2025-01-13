use crate::events::asynchronous::AsyncEvent;
use crate::mc_server::{SERVER_PROTOCOL_VERSION, SERVER_VERSION};
use crate::utils::errors::Error;
use crate::utils::files::FILES;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crab_macros::AsyncEvent;
use crab_protocol::packets::status::{PlayersInfo, ServerInfo, ServerVersion};
use evenio::event::GlobalEvent;
use std::fs;
use std::net::IpAddr;
use tracing::warn;

const BASE64_ENCODING_PREFIX: &str = "data:image/png;base64,";
const SERVER_ICON_PATH: &str = "server-icon.png";

/// Fired when player views server in a server list.
/// Used to show MOTD, online players, players sample etc.
///
/// Based on Bukkit's [ServerListPingEvent](https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/server/ServerListPingEvent.html)
#[derive(GlobalEvent, AsyncEvent, Debug)]
pub struct ServerListPingEvent {
    ip_address: IpAddr,
    pub server_info: ServerInfo,
}

impl ServerListPingEvent {
    pub fn new(ip_address: IpAddr, server_info: ServerInfo) -> Self {
        ServerListPingEvent {
            ip_address,
            server_info,
        }
    }

    pub fn with_default_server_info(ip_address: IpAddr) -> Self {
        let server_icon_path = FILES.server_root.join(SERVER_ICON_PATH);
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
            description: "CrabCraft".into(),
            favicon,
            enforces_secure_chat: false,
            previews_chat: false,
        };

        ServerListPingEvent::new(ip_address, server_info)
    }

    #[allow(unused)]
    pub fn ip_address(&self) -> &IpAddr {
        &self.ip_address
    }
}
