use crab_protocol::{packets::common::KeepAliveResponseC2SPacket, raw_packet::RawPacket};

use crate::network::connection::NetworkConnection;

pub struct PlayHandler {
    connection: NetworkConnection,
}

impl PlayHandler {
    pub fn new(connection: NetworkConnection) -> Self {
        Self { connection }
    }

    pub fn handle_packet(&self, raw_packet: RawPacket) {
        if let Some(packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>(self.connection.get_state()) {
            let mut keepalive_state = self.connection.keepalive_state.lock().unwrap();
            if keepalive_state.keepalive_pending && keepalive_state.last_keepalive_time == packet.keep_alive_id {
                keepalive_state.keepalive_pending = false;
            } else {
                self.connection.disconnect("Timeout!".to_owned());
            }
        }
    }
}
