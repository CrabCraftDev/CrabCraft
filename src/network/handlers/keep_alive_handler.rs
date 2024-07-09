use crab_protocol::packets::common::KeepAliveResponseC2SPacket;

use crate::network::connection::NetworkConnection;

pub fn handle_packet(connection: &NetworkConnection, packet: KeepAliveResponseC2SPacket) {
    let mut keep_alive_state = connection.keep_alive_state.lock().unwrap();
    if keep_alive_state.keep_alive_pending && keep_alive_state.last_keep_alive_time == packet.keep_alive_id {
        keep_alive_state.keep_alive_pending = false;
    } else {
        connection.disconnect("Timeout!".to_owned());
    }
}
