use crab_config::configuration_entry::ConfigurationEntry;

pub fn get_test_configuration_entry() -> ConfigurationEntry {
    ConfigurationEntry::new(
        "root".to_string(),
        Some(
            [
                ConfigurationEntry::new("1".to_string(), None, Some("Test".to_string()), false),
                ConfigurationEntry::new("12".to_string(), None, Some("Test".to_string()), false),
                ConfigurationEntry::new("123".to_string(), None, Some("Test".to_string()), false),
                ConfigurationEntry::new(
                    "1234".to_string(),
                    Some(
                        [ConfigurationEntry::new(
                            "12345".to_string(),
                            None,
                            Some("Test".to_string()),
                            false,
                        )]
                        .to_vec(),
                    ),
                    None,
                    false,
                ),
            ]
            .to_vec(),
        ),
        None,
        true,
    )
}
