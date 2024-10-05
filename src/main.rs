use std::process::exit;
use std::{backtrace::Backtrace, panic, thread};

use chrono::Local;
use crab_registry::{RegistryManager, REGISTRY};
use crab_world::blocks::init_blocks;
use crab_world::blocks::{Air, BlockType, CaveAir, VoidAir, BLOCKS};
use log::{error, info, LevelFilter};
use utils::{errors::print_error_report_instructions, logger::Logger};

use crate::mc_server::McServer;
use crate::utils::files::FILES;

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

    let main_thread_id = thread::current().id();
    panic::set_hook(Box::new(move |panic_info| {
        if thread::current().id() != main_thread_id {
            return;
        }
        error!("");
        error!("!!! SERVER PANICKED !!!");
        error!("");
        error!("Panic info:");
        for line in panic_info.to_string().split('\n') {
            error!("{line}");
        }
        error!("");
        error!("Backtrace:");
        for line in Backtrace::force_capture().to_string().split('\n') {
            error!("{line}");
        }
        print_error_report_instructions();
        exit(1)
    }));

    log::set_max_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    FILES.create_directories().expect("Failed to create server directories");

    log::set_logger(&Logger).expect("Failed to prepare logger");
    Logger::rename_log_file().expect("Failed to prepare logger file");

    REGISTRY.get_or_init(RegistryManager::default);
    BLOCKS.get_or_init(init_blocks);

    assert!(CaveAir::default_state_id().unwrap().is_air());
    assert!(VoidAir::default_state_id().unwrap().is_air());
    assert!(Air::default_state_id().unwrap().is_air());

    let mc_server = McServer::new("127.0.0.1:25565".to_string());

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    mc_server.start().await;

    info!("Exiting!");
}
