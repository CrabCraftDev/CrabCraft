use std::process::exit;
use std::{backtrace::Backtrace, panic, thread};

use chrono::Local;
use crab_craft::utils::{errors::print_error_report_instructions, log};
use crab_registry::{REGISTRY, RegistryManager};
use crab_world::blocks::init_blocks;
use crab_world::blocks::{Air, BLOCKS, BlockType, CaveAir, VoidAir};
use tracing::{error, info, info_span};

use crab_craft::mc_server::McServer;
use crab_craft::utils::files::FILES;
use crab_craft::world::OVERWORLD_IDENTIFIER;
use crab_craft::world::worlds_storage::WORLDS_STORAGE;

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_millis();

    FILES.create_directories().expect("Failed to create server directories");

    // Guards make sure all logs are flushed in case of panic
    let _guards = log::init();

    let span = info_span!("Init");
    let span_guard = span.enter();

    set_panic_hook();

    REGISTRY.get_or_init(RegistryManager::default);
    BLOCKS.get_or_init(init_blocks);
    WORLDS_STORAGE.init_dimension(OVERWORLD_IDENTIFIER.clone(), -64);

    assert!(CaveAir::default_state_id().unwrap().is_air());
    assert!(VoidAir::default_state_id().unwrap().is_air());
    assert!(Air::default_state_id().unwrap().is_air());

    let mc_server = McServer::new("127.0.0.1:25565".to_string());

    let startup_finish = Local::now().timestamp_millis();
    info!("CrabCraft started in {}ms!", startup_finish - startup_begin);

    drop(span_guard);

    mc_server.start().await;

    let span = info_span!("Exit");
    let _span_guard = span.enter();

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
