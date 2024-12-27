use crate::configurable::ConfigVersion;
use crate::node::ConfigurationNode;

/// A `Migration` applies custom logic to upgrade configuration files
/// from an older version to the latest version.
///
/// Migrations are triggered when the user's version is in the range (A, B],
/// where:
/// - A is the user's current config version
/// - B is the target (latest) version
pub trait Migration<V: ConfigVersion> {
    /// Applies migration logic to the given configuration node
    /// Note: This code should never panic, try to handle all cases
    fn up(&self, configuration_node: &mut ConfigurationNode);

    /// Returns the version this migration upgrades to
    fn version(&self) -> V;
}
