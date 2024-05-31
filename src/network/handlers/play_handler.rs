use crab_protocol::packets::play::KeepAliveResponseC2SPacket;
use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::utils::errors::Error;

pub struct PlayHandler;

impl PlayHandler {
    pub async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error> {
        if let Some(packet) = raw_packet.decode::<KeepAliveResponseC2SPacket>() {
            if connection.keepalive_pending && connection.last_keepalive_time == packet.keep_alive_id {
                connection.keepalive_pending = false;
            } else {
                connection.disconnect("Timeout!").await;
            }
        }

        Ok(())
    }
}
