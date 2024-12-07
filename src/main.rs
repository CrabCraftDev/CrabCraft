use std::process::exit;
use std::{backtrace::Backtrace, panic, thread};

use chrono::Local;
use crab_registry::{RegistryManager, REGISTRY};
use crab_world::blocks::init_blocks;
use crab_world::blocks::{Air, BlockType, CaveAir, VoidAir, BLOCKS};
use tracing::{error, info};
use utils::{errors::print_error_report_instructions, log};

use crate::mc_server::McServer;
use crate::utils::files::FILES;

mod components;
mod events;
mod handlers;
mod mc_server;
mod network;
mod utils;
mod world;

pub const GITHUB_ISSUES_URL: &str = "https://github.com/CrabCraftDev/CrabCraft/issues/new/choose";
pub const DISCORD_INVITE_URL: &str = "https://discord.gg/XZKprmznqP";

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_millis();

    FILES.create_directories().expect("Failed to create server directories");

    // Guards make sure all logs are flushed in case of panic
    let _guards = log::init();

    set_panic_hook();

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

fn set_panic_hook() {
    let main_thread_id = thread::current().id();
    panic::set_hook(Box::new(move |panic_info| {
        let is_main_thread = thread::current().id() == main_thread_id;
        error!("");
        if is_main_thread {
            error!("!!! SERVER PANICKED !!!");
        } else {
            error!("!!! SERVER SUB-TASK PANICKED !!!");
        }
        error!("");
        error!("Panic info:");
        for line in panic_info.to_string().split('\n') {
            error!("{line}");
        }

        error!("");
        error!("Backtrace:");
        for line in Backtrace::force_capture().to_string().split("\n") {
            error!("{line}");
        }

        if is_main_thread {
            print_error_report_instructions();
            exit(1);
        }
    }));
}
