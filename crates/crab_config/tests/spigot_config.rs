use crab_macros::Config;
use serde::{Deserialize, Serialize};

// GENERATED FROM AI FOR TESTING PURPOSES
// NOT TESTED
// USED FOR TESTING MACRO COMPILATION
#[derive(Config)]
pub struct SpigotConfig {
    pub config_version: u8,
    messages: Messages,
    /// 69420
    settings: Settings,
    commands: Commands,
    stats: Stats,
    world_settings: WorldSettings,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Messages {
    whitelist: String,
    #[serde(rename = "unknown-command")]
    unknown_command: String,
    #[serde(rename = "server-full")]
    server_full: String,
    #[serde(rename = "outdated-client")]
    outdated_client: String,
    #[serde(rename = "outdated-server")]
    outdated_server: String,
    restart: String,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Settings {
    #[serde(rename = "save-user-cache-on-stop-only")]
    save_user_cache_on_stop_only: bool,
    attribute: Attribute,
    bungeecord: bool,
    #[serde(rename = "late-bind")]
    late_bind: bool,
    #[serde(rename = "sample-count")]
    sample_count: u8,
    #[serde(rename = "player-shuffle")]
    player_shuffle: u8,
    #[serde(rename = "filter-creative-items")]
    filter_creative_items: bool,
    #[serde(rename = "user-cache-size")]
    user_cache_size: u32,
    #[serde(rename = "int-cache-limit")]
    int_cache_limit: u32,
    #[serde(rename = "moved-wrongly-threshold")]
    moved_wrongly_threshold: f32,
    #[serde(rename = "moved-too-quickly-multiplier")]
    moved_too_quickly_multiplier: f32,
    #[serde(rename = "item-dirty-ticks")]
    item_dirty_ticks: u32,
    #[serde(rename = "timeout-time")]
    timeout_time: u32,
    #[serde(rename = "restart-on-crash")]
    restart_on_crash: bool,
    #[serde(rename = "restart-script")]
    restart_script: String,
    #[serde(rename = "netty-threads")]
    netty_threads: u8,
    debug: bool,
}

#[derive(Config, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Attribute {
    maxHealth: MaxAttribute,
    movementSpeed: MaxAttribute,
    attackDamage: MaxAttribute,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct MaxAttribute {
    max: f32,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Commands {
    #[serde(rename = "tab-complete")]
    tab_complete: u8,
    log: bool,
    #[serde(rename = "silent-commandblock-console")]
    silent_commandblock_console: bool,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Stats {
    #[serde(rename = "disable-saving")]
    disable_saving: bool,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct WorldSettings {
    default: DefaultWorldSettings,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct DefaultWorldSettings {
    verbose: bool,
    growth: Growth,
    #[serde(rename = "entity-activation-range")]
    entity_activation_range: EntityActivationRange,
    #[serde(rename = "entity-tracking-range")]
    entity_tracking_range: EntityTrackingRange,
    #[serde(rename = "ticks-per")]
    ticks_per: TicksPer,
    #[serde(rename = "hopper-amount")]
    hopper_amount: u8,
    #[serde(rename = "random-light-updates")]
    random_light_updates: bool,
    #[serde(rename = "save-structure-info")]
    save_structure_info: bool,
    #[serde(rename = "mob-spawn-range")]
    mob_spawn_range: u8,
    #[serde(rename = "max-entity-collisions")]
    max_entity_collisions: u8,
    #[serde(rename = "dragon-death-sound-radius")]
    dragon_death_sound_radius: u8,
    #[serde(rename = "seed-village")]
    seed_village: i64,
    #[serde(rename = "seed-feature")]
    seed_feature: i64,
    hunger: Hunger,
    #[serde(rename = "max-tnt-per-tick")]
    max_tnt_per_tick: u8,
    #[serde(rename = "max-tick-time")]
    max_tick_time: MaxTickTime,
    #[serde(rename = "item-despawn-rate")]
    item_despawn_rate: u32,
    #[serde(rename = "merge-radius")]
    merge_radius: MergeRadius,
    #[serde(rename = "arrow-despawn-rate")]
    arrow_despawn_rate: u32,
    #[serde(rename = "enable-zombie-pigmen-portal-spawns")]
    enable_zombie_pigmen_portal_spawns: bool,
    #[serde(rename = "wither-spawn-sound-radius")]
    wither_spawn_sound_radius: u8,
    #[serde(rename = "view-distance")]
    view_distance: u8,
    #[serde(rename = "hanging-tick-frequency")]
    hanging_tick_frequency: u32,
    #[serde(rename = "zombie-aggressive-towards-villager")]
    zombie_aggressive_towards_villager: bool,
    #[serde(rename = "nerf-spawner-mobs")]
    nerf_spawner_mobs: bool,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Growth {
    #[serde(rename = "cactus-modifier")]
    cactus_modifier: u8,
    #[serde(rename = "cane-modifier")]
    cane_modifier: u8,
    #[serde(rename = "melon-modifier")]
    melon_modifier: u8,
    #[serde(rename = "mushroom-modifier")]
    mushroom_modifier: u8,
    #[serde(rename = "pumpkin-modifier")]
    pumpkin_modifier: u8,
    #[serde(rename = "sapling-modifier")]
    sapling_modifier: u8,
    #[serde(rename = "wheat-modifier")]
    wheat_modifier: u8,
    #[serde(rename = "netherwart-modifier")]
    netherwart_modifier: u8,
    #[serde(rename = "vine-modifier")]
    vine_modifier: u8,
    #[serde(rename = "cocoa-modifier")]
    cocoa_modifier: u8,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct EntityActivationRange {
    animals: u8,
    monsters: u8,
    misc: u8,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct EntityTrackingRange {
    players: u8,
    animals: u8,
    monsters: u8,
    misc: u8,
    other: u8,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct TicksPer {
    #[serde(rename = "hopper-transfer")]
    hopper_transfer: u8,
    #[serde(rename = "hopper-check")]
    hopper_check: u8,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct Hunger {
    #[serde(rename = "walk-exhaustion")]
    walk_exhaustion: f32,
    #[serde(rename = "sprint-exhaustion")]
    sprint_exhaustion: f32,
    #[serde(rename = "combat-exhaustion")]
    combat_exhaustion: f32,
    #[serde(rename = "regen-exhaustion")]
    regen_exhaustion: f32,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct MaxTickTime {
    #[default_value(200)]
    tile: u8,
    entity: u8,
}

#[derive(Config, Serialize, Deserialize, Debug)]
struct MergeRadius {
    item: f32,
    exp: f32,
}
