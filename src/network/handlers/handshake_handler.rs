use async_trait::async_trait;
use std::cmp::Ordering;

use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::protocol::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::utils::errors::Error;
use crate::{SERVER_PROTOCOL_VERSION, SERVER_VERSION};

pub struct HandshakeHandler;

#[async_trait]
impl PacketHandler for HandshakeHandler {
    async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
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

        Ok(())
    }
}
