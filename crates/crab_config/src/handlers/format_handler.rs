use crate::configuration_entry::ConfigurationEntry;
use crate::errors::Error;

pub trait ConfigFormatHandler {
    fn read(&self, file_path: String) -> Result<ConfigurationEntry, Error>;
    fn write(&self, file_path: String, contents: ConfigurationEntry) -> Result<(), Error>;
}
