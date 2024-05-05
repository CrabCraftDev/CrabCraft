use async_trait::async_trait;

use crab_protocol::raw_packet::RawPacket;

use crate::network::connection::Connection;
use crate::utils::errors::Error;

#[async_trait]
pub trait PacketHandler {
    async fn handle_packet(connection: &mut Connection, raw_packet: RawPacket) -> Result<(), Error>;
}
