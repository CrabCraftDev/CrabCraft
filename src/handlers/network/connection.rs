use crate::components::name::Name;
use crate::components::uuid::UuidComponent;
use crate::events::player::player_quit::PlayerQuitEvent;
use crate::network::connection::NetworkConnection;
use crab_protocol::packets::play::{PlayerInfoRemoveS2CPacket, RemoveEntitiesS2CPacket, SystemChatMessageS2CPacket};
use crab_text::text::Text;
use evenio::fetch::Fetcher;
use evenio::{
    entity::EntityId,
    event::{Despawn, Receiver, Sender},
};
use tracing::info;

pub fn on_quit(
    receiver: Receiver<PlayerQuitEvent, (EntityId, &Name, &NetworkConnection, &UuidComponent)>,
    sender: Sender<Despawn>,
    fetcher: Fetcher<&NetworkConnection>,
) {
    let (entity, name, network_connection, uuid) = receiver.query;

    network_connection.close();
    info!("{name} left the game: {}", receiver.event.quit_reason);
    sender.send_to(entity, Despawn);
    let leave_message = Text::from(format!("{name} left the game!"));
    for conn in fetcher {
        conn.send_packet(RemoveEntitiesS2CPacket {
            entities_ids: vec![entity.index().0 as i32],
        });
        conn.send_packet(PlayerInfoRemoveS2CPacket {
            players_to_remove: vec![uuid.0],
        });
        conn.send_packet(SystemChatMessageS2CPacket {
            content: leave_message.clone(),
            is_actionbar: false,
        })
    }
}
