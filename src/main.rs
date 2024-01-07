use log::{info, LevelFilter};

use network::tcp_server::TcpServer;
use utils::logger::Logger;

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

    let server = TcpServer::start("127.0.0.1:25565".to_string()).await;
    let handle = server.lock().await.thread.take().expect("No thread handle");

    handle.await.expect("Server had unexpected error!!");

    info!("CrabCraft server disabled!");
}
