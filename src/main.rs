use chrono::Local;
use log::{info, LevelFilter};
use tokio::select;

use network::tcp_server::TcpServer;
use utils::logger::Logger;

use crate::mc_server::McServer;

mod network;
mod utils;

mod mc_server;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_millis();
    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    log::set_logger(&Logger).unwrap();
    Logger::rename_log_file().expect("Failed to prepare logger! Exiting!");

    let tcp_server = TcpServer::new("127.0.0.1:25565".to_string());
    let server_future = tcp_server.start();

    let mut mc_server = McServer::new(&tcp_server);
    let mc_server_future = mc_server.tick_loop();

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    select! {
        _ = mc_server_future => {}
        _ = server_future => {}
    }

    // Shutdown
    mc_server.do_shutdown();
    info!("Exiting!");
}
