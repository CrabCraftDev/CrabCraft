use crate::utils::TestConfig;
use crab_config::configurable::Configurable;
use crab_config::configuration_entry::ConfigurationEntry;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::version_control::modifications::processor::Processor;
use serde_yml::{Number, Value};
use std::fs;

const TEST_FILE: &str = "processor_config_test.yml";

#[test]
fn processor_test() {
    let configurable = Configurable::<TestConfig>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable
        .read_and_save_default_if_config_doesnt_exists(Vec::new())
        .unwrap();

    let example_processor = Processor::new(
        0,
        Box::new(|x: &mut ConfigurationEntry| -> &mut ConfigurationEntry {
            x.set_value_by_flat_path("version", Value::Number(Number::from(1)));
            x.set_value_by_flat_path("test", Value::Number(Number::from(69420)));
            x
        }),
    );

    let another_example_processor = Processor::new(
        1,
        Box::new(|x: &mut ConfigurationEntry| -> &mut ConfigurationEntry {
            x.set_value_by_flat_path("version", Value::Number(Number::from(2)));
            x
        }),
    );

    let processor_list = vec![another_example_processor, example_processor];
    let test_config = configurable.read_config_and_verify(processor_list).unwrap();

    assert_eq!(test_config.test, 69420);

    assert_eq!(test_config.version, 2);
    fs::remove_file(TEST_FILE).unwrap();
}
