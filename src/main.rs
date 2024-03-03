use chrono::Local;
use log::{info, LevelFilter};

use utils::logger::Logger;

use crate::mc_server::McServer;

mod network;
mod utils;

mod mc_server;
#[cfg(test)]
#[path = "../tests/mod.rs"]
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

    let mc_server = McServer::new("127.0.0.1:25565".to_string());

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    mc_server.start().await;

    info!("Exiting!");
}
