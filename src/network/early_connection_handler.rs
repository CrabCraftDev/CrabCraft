use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crab_protocol::packets::common::{PingRequestC2SPacket, PingResponseS2CPacket};
use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::packets::status::{
    PlayersInfo, ServerInfo, ServerVersion, StatusRequestC2SPacket, StatusResponseS2CPacket,
};
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;
use kanal::Sender;
use log::debug;
use std::cmp::Ordering;
use std::fs;

use crate::utils::fs::get_server_path;
use crate::{
    mc_server::{SERVER_PROTOCOL_VERSION, SERVER_VERSION},
    network::connection::Connection,
};

/// Handles packets in Status or Handshake states
pub struct EarlyConnectionHandler;

impl EarlyConnectionHandler {
    pub fn handle_status_packet(connection: &mut Connection, raw_packet: RawPacket) {
        let state = connection.state();
        if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>(state) {
            connection.send_packet(PingResponseS2CPacket { id: packet.id });
        }

        let Some(_packet) = raw_packet.decode::<StatusRequestC2SPacket>(state) else {
            return;
        };

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
                sample: vec![],
            },
            description: "CrabCraft!".to_string(),
            favicon,
            enforces_secure_chat: false,
            previews_chat: false,
        };

        connection.send_packet(StatusResponseS2CPacket { server_info });

        debug!("Pinged server list!");
    }

    pub fn handle_handshake_packet(
        connection: &mut Connection,
        raw_packet: RawPacket,
        new_player_sender: Sender<Connection>,
    ) {
        let state = connection.state();
        let Some(packet) = raw_packet.decode::<HandshakeC2SPacket>(state) else {
            return;
        };

        match packet.next_state {
            NextState::Status => return connection.set_state(ProtocolState::Status),
            NextState::Login => connection.set_state(ProtocolState::Login),
            NextState::Transfer => return connection.disconnect("CrabCraft doesn't support transfers!"),
        }

        let message = match packet.protocol_version.cmp(&SERVER_PROTOCOL_VERSION) {
            Ordering::Less => format!("Server is on newer Minecraft version! Use {SERVER_VERSION}"),
            Ordering::Greater => format!("Server is on older Minecraft version! Use {SERVER_VERSION}"),
            Ordering::Equal => return new_player_sender.send(connection.clone()).unwrap(),
        };
        connection.disconnect(&message);
    }
}
