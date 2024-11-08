use crate::entity::player::LastTeleport;
use crate::{
    entity::{
        components::{name::Name, uuid::Uuid},
        player::Player,
    },
    network::connection::NetworkConnection,
};
use crab_protocol::packets::login::LoginFinishedS2CPacket;
use crab_protocol::types::position::{Position, Rotation, Vec3Double, Velocity};
use evenio::{
    event::{EventMut, GlobalEvent, Insert, ReceiverMut, Sender, Spawn},
    fetch::Fetcher,
};
use uuid::Uuid as ActualUuid;

#[derive(Debug, GlobalEvent)]
pub struct IncomingConnection {
    pub connection: NetworkConnection,
    pub player_name: String,
    pub player_uuid: ActualUuid,
}

#[allow(clippy::type_complexity)]
pub fn on_incoming_connection(
    receiver: ReceiverMut<IncomingConnection>,
    mut sender: Sender<(
        Spawn,
        Insert<Player>,
        Insert<NetworkConnection>,
        Insert<Name>,
        Insert<Uuid>,
        Insert<Velocity>,
        Insert<Position>,
        Insert<Rotation>,
        Insert<LastTeleport>,
    )>,
    fetcher: Fetcher<(&Player, &Name, &Uuid)>,
) {
    let IncomingConnection {
        connection,
        player_name,
        player_uuid,
    } = EventMut::take(receiver.event);

    if fetcher
        .iter()
        .any(|(_, name, uuid)| name.0 == player_name || uuid.0 == player_uuid)
    {
        connection.disconnect("You are already logged in from another location!".to_owned());
        return;
    }

    connection.set_prefix(&player_name);
    connection.send_packet(LoginFinishedS2CPacket {
        uuid: player_uuid,
        username: player_name.clone(),
        properties: Vec::new(),
    });

    let player = sender.spawn();
    sender.insert(player, Player);
    sender.insert(player, connection);
    sender.insert(player, Uuid(player_uuid));
    sender.insert(player, Name(player_name));

    sender.insert::<Velocity>(player, Vec3Double::default().into());
    sender.insert(player, Position::new(100.0, 70.0, 100.0));
    sender.insert(player, Rotation::default());
    sender.insert(player, LastTeleport::default());
}
