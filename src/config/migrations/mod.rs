mod m0_0_3_rename_crab_craft_usage;

use crab_config::SemverVersion;
use crab_config::version_control::migrations::migration::Migration;

pub fn up() -> Box<dyn Migration<SemverVersion>> {
    Box::new(m0_0_3_rename_crab_craft_usage::ConfigMigration)
}
