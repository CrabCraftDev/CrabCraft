use crate::error::ConfigError;
use crate::node::ConfigurationNode;

pub trait ConfigFormatHandler {
    fn read(&self, file_path: String) -> Result<ConfigurationNode, ConfigError>;
    fn write(&self, file_path: String, contents: ConfigurationNode) -> Result<(), ConfigError>;
}
