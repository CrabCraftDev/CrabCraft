use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use crab_macros::Config;
use semver::Version;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "test_enum_documentation_persistence.yaml";

const EXPECTED_FILE_CONTENT: &str = r#"# Popular compression types
# Default value: Gzip
# Possible values:
#  - Gzip - Gzip is a widely used compression algorithm based on the DEFLATE algorithm.
#  - Bzip2 - Bzip2 is a high-compression algorithm using the Burrows-Wheeler transform.
#  - Lzma - Lzma offers high compression ratios and is used in formats like 7z.
#  - Zstd - Zstd is a fast compression algorithm providing good ratios and speed.
compression: gzip
"#;

#[derive(Default, Config, Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
enum TestEnum {
    #[default]
    /// Gzip is a widely used compression algorithm based on the DEFLATE algorithm.
    Gzip,
    /// Bzip2 is a high-compression algorithm using the Burrows-Wheeler transform.
    Bzip2,
    /// Lzma offers high compression ratios and is used in formats like 7z.
    Lzma,
    /// Zstd is a fast compression algorithm providing good ratios and speed.
    Zstd,
}

#[derive(Config, Deserialize, Serialize, PartialEq, Debug)]
struct TestConfig {
    #[default_value(TestEnum::Gzip)]
    /// Popular compression types
    compression: TestEnum,
}

#[test]
fn enum_documentation_persistence() {
    // Setup
    let config = TestConfig::default();
    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable =
        Configurable::<crate::utils::TestConfig, Version>::new(CONFIG_FILE.to_string(), Box::new(yaml_handler));

    configurable.write_node(config.as_configuration_node()).unwrap();

    // Act
    let file_content = std::fs::read_to_string(CONFIG_FILE).unwrap();
    assert_eq!(file_content, EXPECTED_FILE_CONTENT);

    // Cleanup

    // std::fs::remove_file(CONFIG_FILE).unwrap();
}
