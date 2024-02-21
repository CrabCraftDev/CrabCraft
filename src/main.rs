use chrono::Local;
use log::{info, LevelFilter};

use network::tcp_server::TcpServer;
use utils::logger::Logger;
use crate::mc_server::McServer;

mod network;
mod utils;

#[cfg(test)]
mod tests;
mod mc_server;

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_micros();
    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    log::set_logger(&Logger).unwrap()
    Logger::rename_log_file().expect("Failed to prepare logger! Exiting!");
    
    let tcp_server = TcpServer::new("127.0.0.1:25565".to_string());
    let mut mc_server = McServer::new(&tcp_server);
    let server_future = tcp_server.start();
    let startup_finish = Local::now().timestamp_micros();
    
    info!("CrabCraft started in {}ms!", ((startup_finish - startup_begin) / 100) as f32 / 10.);
    
    mc_server.tick_loop().await;

    // Shutdown
    mc_server.shutdown();
    server_future.await;

    info!("CrabCraft server disabled!");
}
