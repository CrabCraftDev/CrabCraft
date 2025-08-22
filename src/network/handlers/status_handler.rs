use crate::events::global::server_list_ping::ServerListPingEvent;
use crate::network::connection::NetworkConnection;
use crab_protocol::packets::common::{PingRequestC2SPacket, PingResponseS2CPacket};
use crab_protocol::packets::status::{StatusRequestC2SPacket, StatusResponseS2CPacket};
use crab_protocol::raw_packet::RawPacket;

pub async fn handle_packet(connection: &NetworkConnection, raw_packet: RawPacket) {
    let state = connection.state();
    if let Some(packet) = raw_packet.decode::<PingRequestC2SPacket>(state) {
        connection.send_packet(PingResponseS2CPacket { id: packet.id });
        connection.close();
    }

    if raw_packet.decode::<StatusRequestC2SPacket>(state).is_some() {
        let event = connection
            .send_event_with_response(ServerListPingEvent::with_default_server_info(connection.ip_address))
            .await
            .unwrap();

        connection.send_packet(StatusResponseS2CPacket {
            server_info: event.server_info,
        });
    }
}
