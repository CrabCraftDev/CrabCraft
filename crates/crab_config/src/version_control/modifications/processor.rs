use crate::configuration_entry::ConfigurationEntry;

pub struct Processor {
    config_version: u32,
    pub(crate) processor_function: Box<dyn Fn(&mut ConfigurationEntry) -> &mut ConfigurationEntry>,
}

impl Processor {
    pub fn new(
        config_version: u32,
        processor_function: Box<dyn Fn(&mut ConfigurationEntry) -> &mut ConfigurationEntry>,
    ) -> Self {
        Processor {
            config_version,
            processor_function,
        }
    }

    /// Describes at which config version we want to apply changes from
    /// `get_processor_function`
    pub fn get_config_applied_version(&self) -> u32 {
        self.config_version
    }
}
