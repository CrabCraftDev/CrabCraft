use crab_config::SemverVersion;
use crab_config::node::ConfigurationNode;
use crab_config::version_control::migrations::migration::Migration;

pub struct ConfigMigration;

impl Migration<SemverVersion> for ConfigMigration {
    fn up(&self, configuration: &mut ConfigurationNode) {
        let value = configuration.get_value_from_flat_path("mention_crabcraft_usage");
        if let Some(value) = value {
            configuration.set_value_by_flat_path("mention_crab_craft_usage", value.clone());
        }
    }

    fn version(&self) -> SemverVersion {
        SemverVersion::new(0, 0, 3)
    }
}
