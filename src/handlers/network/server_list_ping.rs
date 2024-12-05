use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::uuid::UuidComponent;
use crate::events::global::server_list_ping::ServerListPingEvent;
use crab_protocol::packets::common::ClientInformation;
use crab_protocol::packets::status::PlayerSample;
use evenio::event::ReceiverMut;
use evenio::fetch::Fetcher;
use std::string::ToString;
use uuid::Uuid;

const MAXIMUM_SIZE_OF_PLAYERS_SAMPLE: usize = 12;

pub fn on_server_list_ping(
    mut receiver: ReceiverMut<ServerListPingEvent>,
    fetcher: Fetcher<(&Player, &Name, &UuidComponent, &ClientInformation)>,
) {
    let anonymous_player = PlayerSample {
        name: "Anonymous Player".to_string(),
        uuid: Uuid::default().to_string(),
    };

    let amount_of_players = fetcher.iter().len();
    receiver.event.server_info.players.online = amount_of_players as i32;

    let mut player_samples: Vec<PlayerSample> = Vec::new();
    for (_, name, uuid, info) in fetcher.iter().take(MAXIMUM_SIZE_OF_PLAYERS_SAMPLE) {
        if !info.allow_server_listings {
            player_samples.push(anonymous_player.clone());
            continue;
        }

        player_samples.push(PlayerSample {
            name: name.0.clone(),
            uuid: uuid.0.to_string(),
        });
    }

    receiver.event.server_info.players.sample = player_samples;
}
