use std::panic;
use std::process::exit;

use chrono::Local;
use crab_registry::{RegistryManager, REGISTRY};
use crab_world::blocks::init_blocks;
use crab_world::BLOCKS;
use log::{error, info, LevelFilter};
use utils::logger::Logger;

use crate::mc_server::McServer;

mod entity;
mod mc_server;
mod network;
mod utils;
mod world;

pub const GITHUB_ISSUES_URL: &str = "https://github.com/CrabCraftDev/CrabCraft/issues/new/choose";
pub const DISCORD_INVITE_URL: &str = "https://discord.gg/XZKprmznqP";

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_millis();

    panic::set_hook(Box::new(|panic_info| {
        error!("Server PANICKED!!! (looks like a crash, that wasn't expected)");
        for line in panic_info.to_string().split('\n') {
            error!("{line}");
        }
        exit(1)
    }));

    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    log::set_logger(&Logger).expect("Failed to prepare logger! Exiting!");
    Logger::rename_log_file().expect("Failed to prepare logger file! Exiting!");

    REGISTRY.get_or_init(RegistryManager::default);
    BLOCKS.get_or_init(init_blocks);

    let mc_server = McServer::new("127.0.0.1:25565".to_string());

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    mc_server.start().await;

    info!("Exiting!");
}
