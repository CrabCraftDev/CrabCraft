use std::cmp::Ordering;

use crab_protocol::packets::handshake::{HandshakeC2SPacket, NextState};
use crab_protocol::protocol_state::ProtocolState;
use crab_protocol::raw_packet::RawPacket;

use crate::mc_server::SERVER_VERSION;
use crate::{mc_server::SERVER_PROTOCOL_VERSION, network::connection::NetworkConnection};

pub struct HandshakeHandler {
    connection: NetworkConnection,
}

impl HandshakeHandler {
    pub fn new(connection: NetworkConnection) -> Self {
        Self { connection }
    }

    pub fn handle_packet(&self, raw_packet: RawPacket) {
        let state = self.connection.get_state();
        let Some(packet) = raw_packet.decode::<HandshakeC2SPacket>(state) else {
            return;
        };

        let next_state = match packet.next_state {
            NextState::Status => ProtocolState::Status,
            NextState::Login => ProtocolState::Login,
            NextState::Transfer => {
                self.connection
                    .disconnect("CrabCraft doesn't support transfers!".to_owned());
                return;
            }
        };

        self.connection.set_state(next_state);

        if next_state != ProtocolState::Login {
            return;
        }

        let message = match packet.protocol_version.cmp(&SERVER_PROTOCOL_VERSION) {
            Ordering::Less => format!("Server is on newer Minecraft version! Use {}", SERVER_VERSION),
            Ordering::Greater => format!("Server is on older Minecraft version! Use {}", SERVER_VERSION),
            Ordering::Equal => return,
        };

        self.connection.disconnect(message);
    }
}
