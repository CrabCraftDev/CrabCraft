use crate::components::name::Name;
use crate::events::player::player_quit::PlayerQuitEvent;
use crate::network::connection::NetworkConnection;
use evenio::{
    entity::EntityId,
    event::{Despawn, Receiver, Sender},
};
use tracing::info;

pub fn on_quit(receiver: Receiver<PlayerQuitEvent, (EntityId, &Name, &NetworkConnection)>, sender: Sender<Despawn>) {
    let (entity, name, network_connection) = receiver.query;

    network_connection.close();
    info!("{name} left the game: {}", receiver.event.quit_reason);
    sender.send_to(entity, Despawn);
}
