use crate::mc_server::McServer;
use chrono::Local;
use crab_registry::RegistryLoader;
use crossbeam::sync::ShardedLock;
use lazy_static::lazy_static;
use log::{error, info, LevelFilter};
use std::panic;
use std::process::exit;
use utils::logger::Logger;

mod chunks;
mod entity;
mod mc_server;
mod network;
mod utils;
mod world;

lazy_static! {
    static ref REGISTRY: ShardedLock<RegistryLoader> = ShardedLock::new(RegistryLoader::default());
}

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
