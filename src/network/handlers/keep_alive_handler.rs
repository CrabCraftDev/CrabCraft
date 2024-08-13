use crab_protocol::{packets::common::KeepAliveResponseC2SPacket, raw_packet::RawPacket};

use crate::network::connection::NetworkConnection;

pub struct KeepAliveHandler {
    connection: NetworkConnection,
}

impl KeepAliveHandler {
    pub fn new(connection: NetworkConnection) -> Self {
        Self { connection }
    }

    pub fn handle_keepalive(&self, raw_packet: RawPacket) {
        if let Some(packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>(self.connection.get_state()) {
            let mut keep_alive_state = self.connection.keep_alive_state.lock().unwrap();
            if keep_alive_state.keep_alive_pending && keep_alive_state.last_keepalive_time == packet.keep_alive_id {
                keep_alive_state.keep_alive_pending = false;
            } else {
                self.connection.disconnect("Timeout!".to_owned());
            }
        }
    }
}
