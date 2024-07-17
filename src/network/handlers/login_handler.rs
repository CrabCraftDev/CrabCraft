use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket, LoginSuccessS2CPacket};
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::NetworkConnection;

pub struct LoginHandler {
    connection: NetworkConnection,
}

impl LoginHandler {
    pub fn new(connection: NetworkConnection) -> Self {
        Self { connection }
    }

    pub fn handle_packet(&self, raw_packet: RawPacket) {
        let state = self.connection.get_state();
        if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>(state) {
            self.connection.send_packet(LoginSuccessS2CPacket {
                uuid: packet.player_uuid,
                username: packet.username,
                properties: Vec::new(),
                strict_protocol_error_handling: true,
            });
        }

        if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>(state) {
            self.connection.set_state(ProtocolState::Configuration);
            self.connection
                .new_player_connection_sender
                .send(self.connection.clone())
                .unwrap();
        }
    }
}
