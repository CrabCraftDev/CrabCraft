#[allow(clippy::module_inception)]
pub mod configuration_entry;
pub mod configuration_entry_add;
pub mod configuration_entry_misc;
mod configuration_entry_remove;
pub mod configuration_entry_search;

#[derive(PartialEq, Debug, Clone)]
pub struct ConfigurationEntry {
    pub key: String,
    pub documentation: Option<String>,
    children: Option<Vec<ConfigurationEntry>>,
    value: Option<String>,
    is_root: bool,
}
