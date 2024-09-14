use crab_protocol::packets::login::{LoginAcknowledgedC2SPacket, LoginStartC2SPacket};
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::{entity::player::network::incoming_connection::IncomingConnection, network::connection::NetworkConnection};

pub fn handle_packet(connection: &NetworkConnection, raw_packet: RawPacket) {
    let state = connection.get_state();
    if let Some(packet) = raw_packet.decode::<LoginStartC2SPacket>(state) {
        // Note: Login success packet is sent from the main thread, so that we can check for duplicate player names
        connection
            .incoming_connection_sender
            .send(IncomingConnection {
                connection: connection.clone(),
                player_name: packet.username,
                player_uuid: packet.player_uuid,
            })
            .unwrap();
    }

    if let Some(_packet) = raw_packet.decode::<LoginAcknowledgedC2SPacket>(state) {
        connection.set_state(ProtocolState::Configuration);
    }
}
