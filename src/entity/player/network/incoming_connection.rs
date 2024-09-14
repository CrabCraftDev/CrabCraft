use crab_protocol::packets::login::LoginSuccessS2CPacket;
use evenio::{
    event::{EventMut, GlobalEvent, Insert, ReceiverMut, Sender, Spawn},
    fetch::Fetcher,
};
use uuid::Uuid as ActualUuid;

use crate::{
    entity::{
        components::{name::Name, uuid::Uuid},
        player::Player,
    },
    network::connection::NetworkConnection,
};

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

    connection.send_packet(LoginSuccessS2CPacket {
        uuid: player_uuid,
        username: player_name.clone(),
        properties: Vec::new(),
        strict_protocol_error_handling: true,
    });

    let player = sender.spawn();
    sender.insert(player, Player);
    sender.insert(player, connection);
    sender.insert(player, Uuid(player_uuid));
    sender.insert(player, Name(player_name));
}
