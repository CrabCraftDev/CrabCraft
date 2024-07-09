use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket};
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::NetworkConnection;

pub fn handle_packet(connection: &NetworkConnection, raw_packet: RawPacket) {
    let state = connection.get_state();
    if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>(state) {
        connection.send_packet(LoginSuccessS2CPacket {
            uuid: packet.player_uuid,
            username: packet.username,
            properties: Vec::new(),
            strict_protocol_error_handling: true,
        });
    }

    if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>(state) {
        connection.set_state(ProtocolState::Configuration);
        connection.new_connection_sender.send(connection.clone()).unwrap();
    }
}
