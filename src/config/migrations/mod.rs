mod m0_0_6_0_rename_crabcraft_usage;

use crab_config::SemverVersion;
use crab_config::version_control::migrations::migration::Migration;

pub fn list() -> Vec<Box<dyn Migration<SemverVersion>>> {
    vec![Box::new(m0_0_6_0_rename_crabcraft_usage::ConfigMigration)]
}
