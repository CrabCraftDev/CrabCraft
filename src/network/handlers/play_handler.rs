use crab_protocol::packets::common::KeepAliveResponseC2SPacket;

use crate::network::connection::Connection;
use crate::utils::errors::Error;

pub struct PlayHandler;

impl PlayHandler {
    pub async fn handle_keep_alive(
        connection: &mut Connection,
        packet: KeepAliveResponseC2SPacket,
    ) -> Result<(), Error> {
        if connection.keepalive_pending && connection.last_keepalive_time == packet.keep_alive_id {
            connection.keepalive_pending = false;
        } else {
            connection.disconnect("Timeout!").await;
        };

        Ok(())
    }
}
