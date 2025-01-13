use crate::components::player::Player;
use crate::events::global::server::ServerShutdownEvent;
use crate::network::connection::NetworkConnection;
use evenio::event::Receiver;
use evenio::fetch::Fetcher;

pub fn server_shutdown(_: Receiver<ServerShutdownEvent>, fetcher: Fetcher<(&Player, &NetworkConnection)>) {
    for (_, network_connection) in fetcher.iter() {
        network_connection.disconnect("Server shutting down!".into());
    }
}
