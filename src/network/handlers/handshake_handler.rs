use std::cmp::Ordering;

use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::protocol::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::{mc_server::SERVER_PROTOCOL_VERSION, network::connection::Connection};
use crate::{mc_server::SERVER_VERSION, utils::errors::Error};

pub struct HandshakeHandler;

impl HandshakeHandler {
    pub async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(packet) = raw_packet.decode::<HandshakeC2SPacket>() {
            match packet.next_state {
                NextState::Status => {
                    let mut state = connection.state.write().unwrap();
                    *state = ProtocolState::Status;
                }
                NextState::Login => {
                    let mut state = connection.state.write().unwrap();
                    *state = ProtocolState::Login;
                }
                NextState::Transfer => {
                    connection.disconnect("CrabCraft doesn't support transfers!").await;
                    return Ok(());
                }
            };

            if *connection.state.read().unwrap() != ProtocolState::Login
                || packet.protocol_version == SERVER_PROTOCOL_VERSION
            {
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

        Ok(())
    }
}
