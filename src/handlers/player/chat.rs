use crab_protocol::packets::play::{ChatMessageC2SPacket, SystemChatMessageS2CPacket};
use crab_text::text::Text;
use evenio::{event::Receiver, fetch::Fetcher};
use tracing::{info, info_span};

use crate::{components::name::Name, events::player::packet::PacketEvent, network::connection::NetworkConnection};

pub fn chat_packet_receiver(receiver: Receiver<PacketEvent, &Name>, connections: Fetcher<&NetworkConnection>) {
    let name = receiver.query;

    let state = receiver.event.protocol_state;
    let packet = &receiver.event.packet;

    let Some(packet) = packet.decode::<ChatMessageC2SPacket>(state) else {
        return;
    };

    let message = format!("<{name}> {}", packet.message);

    info_span!("Chat").in_scope(|| {
        info!(message);
    });

    let message = Text::from(message);
    for connection in connections {
        connection.send_packet(SystemChatMessageS2CPacket {
            content: message.clone(),
            is_actionbar: false,
        });
    }
}
