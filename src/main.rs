use chrono::Local;
use log::{error, info, LevelFilter};
use std::panic;
use std::process::exit;

use utils::logger::Logger;

use crate::mc_server::McServer;

mod chunks;
mod mc_server;
mod network;
mod utils;

pub const SERVER_PROTOCOL_VERSION: i32 = 765;
pub const SERVER_VERSION: &str = "1.20.4";

#[tokio::main]
async fn main() {
    panic::set_hook(Box::new(|panic_info| {
        error!("Server PANICKED!!! (looks like a crash, that wasn't expected)");
        for line in panic_info.to_string().split('\n') {
            error!("{}", line);
        }
        exit(1)
    }));

    let startup_begin = Local::now().timestamp_millis();
    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    log::set_logger(&Logger).expect("Failed to prepare logger! Exiting!");
    Logger::rename_log_file().expect("Failed to prepare logger file! Exiting!");

    let mc_server = McServer::new("127.0.0.1:25565".to_string());

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    mc_server.start().await;

    info!("Exiting!");
}
