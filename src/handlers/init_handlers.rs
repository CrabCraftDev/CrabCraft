use crate::handlers::{network, player, server};
use evenio::handler::IntoHandler;
use evenio::prelude::World;

pub fn init_handlers(world: &mut World) {
    // Network
    world.add_handler(network::incoming_connection::on_player_login_event.low());
    world.add_handler(network::connection::on_quit);
    world.add_handler(network::configuration_packet_handler::configuration_packets);
    world.add_handler(network::incoming_connection::player_login_checks);
    world.add_handler(network::server_list_ping::on_server_list_ping);
    world.add_handler(network::block_interaction::on_block_break);

    // Player entity
    world.add_handler(player::position_handler::on_move);
    world.add_handler(player::position_handler::on_teleport);
    world.add_handler(player::position_handler::move_packet_receiver);
    world.add_handler(player::world_join::on_world_join);
    world.add_handler(player::chat::chat_packet_receiver);

    // Server
    world.add_handler(server::server_shutdown);
}
