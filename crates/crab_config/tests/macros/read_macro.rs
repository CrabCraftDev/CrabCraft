use crate::macros::utils::TestConfig;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use serde_yml::Mapping;

#[test]
fn read_macro() {
    let test_file_contents = "\
version: 8
another_one: 8
test_value:
    another_test_value: 8
    ";
    let serde_mapping = serde_yml::from_str::<Mapping>(test_file_contents).unwrap();

    let configuration_entry = YamlHandler::map_values_to_configuration_entry(&serde_mapping).unwrap();
    let cfg = TestConfig::map(configuration_entry);

    assert_eq!(cfg.version, 8);
}
