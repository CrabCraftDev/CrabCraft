use async_trait::async_trait;

use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket};
use crab_protocol::protocol::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::network::packet_handler::PacketHandler;
use crate::utils::errors::Error;

pub struct LoginHandler;

#[async_trait]
impl PacketHandler for LoginHandler {
    async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>() {
            connection
                .send_packet(LoginSuccessS2CPacket {
                    uuid: packet.player_uuid,
                    username: packet.username,
                    properties: Vec::new(),
                    strict_protocol_error_handling: true,
                })
                .await;
        }

        if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>() {
            connection.state = ProtocolState::Configuration;
            return Ok(());
        }

        Ok(())
    }
}
