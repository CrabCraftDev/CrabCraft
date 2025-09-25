use crate::events::player::player_quit::PlayerQuitReason;
use crate::network::connection::NetworkConnection;
use crab_protocol::packets::common::KeepAliveResponseC2SPacket;

pub async fn handle_packet(connection: &NetworkConnection, packet: KeepAliveResponseC2SPacket) {
    let mut keep_alive_state = connection.keep_alive_state.lock();
    if keep_alive_state.pending && keep_alive_state.last_keep_alive_time == packet.keep_alive_id {
        keep_alive_state.pending = false;
    } else {
        connection.disconnect_with_reason(PlayerQuitReason::TimedOut);
    }
}
