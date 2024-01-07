use log::{info, LevelFilter};

use network::tcp_server::TcpServer;
use utils::logger::Logger;

use crate::network::handlers::global_handler::GlobalHandler;
use crate::network::handlers::handshake_handler::HandshakeHandler;

mod network;
mod utils;

#[tokio::main]
async fn main() {
    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    log::set_logger(&Logger).unwrap();
    Logger::rename_log_file().expect("TODO: panic message");

    info!("CrabCraft started!");

    let server = TcpServer::new(
        "127.0.0.1:25565".to_string(),
        vec![Box::new(HandshakeHandler), Box::new(GlobalHandler)],
    );
    server.start().await;

    info!("CrabCraft server disabled!");
}
