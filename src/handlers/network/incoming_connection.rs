use crate::components::player::{LastTeleport, Player};
use crate::components::{name::Name, uuid::Uuid};
use crate::events::global::incoming_connection::IncomingConnection;
use crate::events::player::player_pre_login::PlayerPreLoginEvent;
use crate::events::player::player_pre_login::PlayerPreLoginStatus::KickAnotherLocation;
use crate::network::connection::NetworkConnection;
use crab_protocol::types::position::{Position, Rotation, Vec3Double, Velocity};
use evenio::{
    event::{EventMut, ReceiverMut, Sender, Spawn},
    fetch::Fetcher,
};
use std::sync::Arc;

pub fn on_pre_login_event(mut receiver: ReceiverMut<PlayerPreLoginEvent>, fetcher: Fetcher<(&Player, &Name, &Uuid)>) {
    if fetcher
        .iter()
        .any(|(_, name, uuid)| name.0 == receiver.event.username() || uuid.0 == receiver.event.uuid())
    {
        receiver.event.status = KickAnotherLocation;
    }
}

#[allow(clippy::type_complexity)]
pub fn on_incoming_connection(
    receiver: ReceiverMut<IncomingConnection>,
    sender: Sender<
        Spawn<(
            Name,
            NetworkConnection,
            Player,
            Position,
            Rotation,
            Uuid,
            Velocity,
            LastTeleport,
        )>,
    >,
) {
    let IncomingConnection {
        connection,
        player_name,
        player_uuid,
    } = EventMut::take(receiver.event);

    let entity_id = Arc::clone(&connection.entity_id);
    let id = sender.spawn((
        Name(player_name),
        connection,
        Player,
        Position::new(100.0, 70.0, 100.0),
        Rotation::default(),
        Uuid(player_uuid),
        Velocity(Vec3Double::default()),
        LastTeleport::default(),
    ));
    let mut lock = entity_id.write().unwrap();
    *lock = Some(id);
}
