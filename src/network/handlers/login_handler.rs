use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket};
use crab_protocol::protocol::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::utils::errors::Error;

pub struct LoginHandler;

impl LoginHandler {
    pub async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<bool, Error> {
        let state = *connection.state.read().unwrap();
        if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>(state) {
            connection
                .send_packet(LoginSuccessS2CPacket {
                    uuid: packet.player_uuid,
                    username: packet.username,
                    properties: Vec::new(),
                    strict_protocol_error_handling: true,
                })
                .await;
        }

        if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>(state) {
            let mut state = connection.state.write().unwrap();
            *state = ProtocolState::Configuration;
            return Ok(true);
        }

        Ok(false)
    }
}
