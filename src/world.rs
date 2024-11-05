use crate::{entity::player, network::connection::NetworkConnection};
use evenio::{
    event::{GlobalEvent, Receiver},
    fetch::Fetcher,
    world::World,
};

/// Creates a new World with all the defualt systems (handlers)
pub fn new() -> World {
    let mut world = World::new();

    // Player entity
    world.add_handler(player::network::connection::receive_packets);
    world.add_handler(player::network::connection::on_disconnect);
    world.add_handler(player::network::incoming_connection::on_incoming_connection);
    world.add_handler(player::network::configuration_handler::configuration_packets);
    world.add_handler(player::network::play_handler::on_world_join);

    world.add_handler(server_shutdown);

    // World
    world.add_handler(player::network::play_handler::move_packet_receiver);
    world.add_handler(player::world::position_handler::on_teleport);
    world.add_handler(player::world::position_handler::on_move);

    world
}

#[derive(GlobalEvent)]
pub struct WorldTick;

#[derive(GlobalEvent)]
pub struct ServerShutdown;

fn server_shutdown(_: Receiver<ServerShutdown>, fetcher: Fetcher<&NetworkConnection>) {
    for connection in fetcher.iter() {
        connection.disconnect("Server shutting down!".to_owned());
    }
}
