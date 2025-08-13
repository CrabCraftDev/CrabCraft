use crate::components::player::Player;
use crate::events::global::server::ServerShutdownEvent;
use crate::network::connection::NetworkConnection;
use crate::world::storage::WORLDS_STORAGE;
use evenio::event::Receiver;
use evenio::fetch::Fetcher;
use tracing::info;

pub fn server_shutdown(_: Receiver<ServerShutdownEvent>, fetcher: Fetcher<(&Player, &NetworkConnection)>) {
    info!("Server is shutting down, disconnecting players...");
    for (_, network_connection) in fetcher.iter() {
        network_connection.disconnect("Server shutting down!".into());
    }

    // FIXME: The saving system should be improved/redesigned.
    info!("Saving worlds...");
    WORLDS_STORAGE.save().expect("Failed to save worlds");
}
