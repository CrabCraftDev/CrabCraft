use crate::macros::utils::TestConfig;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::types::config::Config;
use serde_json::{Map, Value};

#[test]
fn read_macro() {
    let test_file_contents = "\
_version: 8
another_one: 8
test_value:
    another_test_value: 8
    ";
    let serde_mapping = serde_saphyr::from_str::<Map<String, Value>>(test_file_contents).unwrap();

    let configuration_node = YamlHandler::map_values_to_configuration_node(&serde_mapping);
    let cfg = TestConfig::from_configuration_node(&configuration_node).unwrap();

    assert_eq!(cfg._version, 8);
}
