use crate::configuration_entry::ConfigurationEntry;
use crate::configuration_entry::configuration_entry::FLAT_PATH_SEPARATOR;
use crate::errors::Error;

/// Searches for `ConfigurationKey` in a tree given following flat key \
/// Given following yaml:
/// ```yml
/// test:
///     source: 2
/// ```
/// You can access `source` `ConfigurationKey` from tree by using flat key `test.source` \
/// Example:
/// ```rust
/// use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;
/// use crab_config::configuration_entry::ConfigurationEntry;
/// // Implementation of above yaml
/// let configuration_key =
/// ConfigurationEntry::new(
///     "root".to_owned(),
///     Some(vec![
///          ConfigurationEntry::new(
///             "test".to_owned(),
///             Some(vec![
///                 ConfigurationEntry::new(
///                 "source".to_owned(),
///                 None,
///                 Some("2".to_owned()),
///                 false
///              )
///          ]),
///          None,
///          false,
///          )
///     ]),
///     None,
///     false
/// );
/// // Usage of the function
/// let value = get_configuration_entry_from_flat_key(&configuration_key, "test.source").unwrap();
/// println!("{}", value.get_value().unwrap());
/// ```
///
pub fn get_configuration_entry_from_flat_key(
    configuration_entry: &ConfigurationEntry,
    key: &str,
) -> Result<ConfigurationEntry, Error> {
    if configuration_entry.is_root {
        return search_for_next_key(configuration_entry, key);
    }

    if key == configuration_entry.key {
        return Ok(configuration_entry.to_owned());
    }

    if configuration_entry.children.is_none() {
        return Err(Error::NoEntriesBelowRoot);
    }
    search_for_next_key(configuration_entry, key)
}

fn search_for_next_key(configuration_entry: &ConfigurationEntry, key: &str) -> Result<ConfigurationEntry, Error> {
    const FIRST_ELEMENT_INDEX: usize = 0;

    let mut split_key: Vec<&str> = key.split(FLAT_PATH_SEPARATOR).collect();
    let first_key = split_key[FIRST_ELEMENT_INDEX];
    let next = configuration_entry.get_next()?;

    for config_entry in next {
        if config_entry.key != first_key {
            continue;
        }

        // In case there is one key left shouldn't remove first key because
        // We already have a solution next recursion will give solution
        if split_key.len() != 1 {
            split_key.remove(FIRST_ELEMENT_INDEX);
        }

        let rest_of_the_key = split_key.join(&*FLAT_PATH_SEPARATOR.to_string());
        let result = get_configuration_entry_from_flat_key(config_entry, &rest_of_the_key);
        return match result {
            Ok(configuration_entry) => Ok(configuration_entry),
            Err(error) => Err(error),
        };
    }

    Err(Error::ElementNotFound)
}
