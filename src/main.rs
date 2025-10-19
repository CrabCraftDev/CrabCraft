use chrono::Local;
use crab_craft::config::manager::ConfigManager;
use crab_craft::utils::{errors::print_error_report_instructions, log};
use crab_registry::{REGISTRY, RootRegistry, init, load_block_entities};
use crab_world::blocks::init_blocks;
use crab_world::blocks::{Air, BlockType, CaveAir, VoidAir};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::{backtrace::Backtrace, panic, thread};
use tracing::{error, info, info_span};

use crab_craft::mc_server::McServer;
use crab_craft::utils::files::FILES;
use crab_craft::world::OVERWORLD_IDENTIFIER;
use crab_craft::world::storage::WORLDS_STORAGE;
use crab_registry::minecraft::{Biome, Empty};
use crab_text::utils::namespaced_key::NamespacedKey;
use crab_world::blocks::block_registry::BlockInformation;

#[tokio::main]
async fn main() {
    let startup_begin = Local::now().timestamp_millis();

    FILES.create_directories().expect("Failed to create server directories");

    // Guards make sure all logs are flushed in case of panic
    let guards = log::init();

    let span = info_span!("Init");
    let span_guard = span.enter();

    set_panic_hook();

    if ConfigManager::init().is_err() {
        drop(guards);
        exit(1);
    }

    let mut registry = RootRegistry::new();

    // init_normal(&mut registry);
    init::<Empty>(&mut registry, NamespacedKey::minecraft("cat_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("chicken_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("cow_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("damage_type").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("dimension_type").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("frog_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("painting_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("pig_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("wolf_sound_variant").unwrap());
    init::<Empty>(&mut registry, NamespacedKey::minecraft("wolf_variant").unwrap());
    init::<Biome>(&mut registry, NamespacedKey::from_str(Biome::REGISTRY.id).unwrap());

    load_block_entities(&mut registry);

    registry.register_registry(BlockInformation::REGISTRY, init_blocks());

    REGISTRY.store(Arc::new(registry));

    WORLDS_STORAGE.init_dimension(OVERWORLD_IDENTIFIER.clone(), -64);

    assert!(CaveAir::default_state_id().unwrap().is_air());
    assert!(VoidAir::default_state_id().unwrap().is_air());
    assert!(Air::default_state_id().unwrap().is_air());

    let mc_server = McServer::new();

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
