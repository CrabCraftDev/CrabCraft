use crate::components::current_world::CurrentWorld;
use crate::components::player::{LastTeleport, Player};
use crate::components::{name::Name, uuid::UuidComponent};
use crate::events::player::player_login::{PlayerLoginEvent, PlayerLoginStatus};
use crate::network::connection::NetworkConnection;
use crate::world::OVERWORLD_IDENTIFIER;
use crab_protocol::types::position::{Position, Rotation, Vec3Double, Velocity};
use evenio::{
    event::{Receiver, ReceiverMut, Sender, Spawn},
    fetch::Fetcher,
};
use std::sync::Arc;

pub fn player_login_checks(
    mut receiver: ReceiverMut<PlayerLoginEvent>,
    fetcher: Fetcher<(&Player, &Name, &UuidComponent)>,
) {
    if fetcher
        .iter()
        .any(|(_, name, uuid)| name.0 == receiver.event.username() || uuid.0 == receiver.event.uuid())
    {
        receiver.event.status =
            PlayerLoginStatus::Disallowed("You are already logged in from another location".to_string());
    }
}

#[allow(clippy::type_complexity)]
pub fn on_player_login_event(
    receiver: Receiver<PlayerLoginEvent>,
    sender: Sender<
        Spawn<(
            Name,
            NetworkConnection,
            Player,
            Position,
            Rotation,
            UuidComponent,
            Velocity,
            LastTeleport,
            CurrentWorld,
        )>,
    >,
) {
    let event = receiver.event;

    if let PlayerLoginStatus::Disallowed(status) = &event.status {
        event.connection.disconnect(status.clone());
        return;
    }

    let entity_id = Arc::clone(&event.connection.entity_id);
    let id = sender.spawn((
        Name(event.username().to_string()),
        event.connection.clone(),
        Player,
        Position::new(100.0, 70.0, 100.0),
        Rotation::default(),
        UuidComponent(event.uuid()),
        Velocity(Vec3Double::default()),
        LastTeleport::default(),
        CurrentWorld(OVERWORLD_IDENTIFIER.clone()),
    ));
    *entity_id.write().unwrap() = Some(id);
}
