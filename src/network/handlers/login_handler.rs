use crab_protocol::packet_transformer::COMPRESSION_THRESHOLD;
use crab_protocol::packets::login::{
    LoginAcknowledgedC2SPacket, LoginFinishedS2CPacket, LoginStartC2SPacket, SetCompressionS2CPacket,
};
use crab_protocol::raw_packet::RawPacket;

use crate::events::player::player_login::{PlayerLoginEvent, PlayerLoginStatus};
use crate::events::player::player_pre_login::{PlayerPreLoginEvent, PlayerPreLoginStatus};
use crate::network::connection::NetworkConnection;
use crate::utils::errors::Error;
use crab_protocol::protocol_state::ProtocolState;

pub async fn handle_packet(connection: &NetworkConnection, raw_packet: RawPacket) -> Result<(), Error> {
    let state = connection.state();
    if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>(state) {
        connection.set_prefix(&packet.username);

        let event = connection
            .send_event_with_response(PlayerPreLoginEvent::new(
                packet.username.to_string(),
                connection.ip_address,
                packet.player_uuid,
            ))
            .await?;

        match event.status {
            PlayerPreLoginStatus::Allowed => {
                let event = connection
                    .send_event_with_response(PlayerLoginEvent::new(
                        packet.username.clone(),
                        packet.player_uuid,
                        connection.clone(),
                    ))
                    .await?;

                if let PlayerLoginStatus::Disallowed(_) = event.status {
                    return Ok(());
                }
                connection.send_packet(SetCompressionS2CPacket {
                    threshold: COMPRESSION_THRESHOLD,
                });

                connection.send_packet(LoginFinishedS2CPacket {
                    uuid: event.uuid(),
                    username: event.username().to_string(),
                    properties: Vec::new(),
                });
            }
            PlayerPreLoginStatus::KickBanned => {
                connection.disconnect("You are banned from this server".into());
            }
            PlayerPreLoginStatus::KickFull => {
                connection.disconnect("The server is full".into());
            }
            PlayerPreLoginStatus::KickWhitelist => {
                connection.disconnect("You are not whitelisted on this server".into());
            }
            PlayerPreLoginStatus::KickOther(_) => {
                connection.disconnect("Other kick reason".into());
            }
        }
    }

    if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>(state) {
        connection.set_state(ProtocolState::Configuration);
    }

    Ok(())
}
