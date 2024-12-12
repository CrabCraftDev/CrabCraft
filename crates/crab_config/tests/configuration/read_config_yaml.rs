use crab_config::configurable::Configurable;
use crab_config::configuration_entry::configuration_entry_search::get_configuration_entry_from_flat_key;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_macros::Config;
use serde::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

const TEST_FILE: &str = "read_config_yaml.yml";
const TEST_FILE2: &str = "read_config_yaml_index_access.yml";

#[derive(Config, Serialize)]
pub struct TestStructure {
    pub test_value_1: String,
    pub test_value_2: String,
    pub test_structure_2: TestStructure2,
}
#[derive(Config, Serialize)]
pub struct TestStructure2 {
    pub test_value_3: i32,
    pub test_structure_3: TestStructure3,
}

#[derive(Config, Serialize)]
pub struct TestStructure3 {
    pub test_value_4: f32,
}

#[test]
fn read_config_yaml_and_verify_using_flattening() {
    // Setup

    const TEST_VALUE_1: &str = "Test1";

    const TEST_VALUE_2: &str = "Test2";

    const TEST_VALUE_3: i32 = 2137;

    const TEST_VALUE_4: f32 = 2.137;

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure>::new(TEST_FILE.to_string(), Box::new(yaml_handler));

    let test_structure_1 = TestStructure {
        test_value_1: TEST_VALUE_1.to_string(),
        test_value_2: TEST_VALUE_2.to_string(),
        test_structure_2: TestStructure2 {
            test_value_3: TEST_VALUE_3,
            test_structure_3: TestStructure3 {
                test_value_4: TEST_VALUE_4,
            },
        },
    };

    create_test_file(test_structure_1, TEST_FILE);

    // Act
    let configuration_entry_result = configurable.read_config();

    // Verify

    match configuration_entry_result {
        Ok(configuration_entry) => {
            let result_map = configuration_entry.serialize_to_flatten_map_with_path();
            match result_map {
                Ok(map) => {
                    assert_eq!(map.get("test_value_1").unwrap(), TEST_VALUE_1);
                    assert_eq!(map.get("test_value_2").unwrap(), TEST_VALUE_2);
                    assert_eq!(
                        map.get("test_structure_2.test_value_3").unwrap(),
                        &TEST_VALUE_3.to_string()
                    );
                    assert_eq!(
                        map.get("test_structure_2.test_structure_3.test_value_4").unwrap(),
                        &TEST_VALUE_4.to_string()
                    );
                }
                Err(error) => {
                    panic!("{}", error);
                }
            }
        }
        Err(error) => {
            panic!("{}", error);
        }
    }
    remove_test_file(TEST_FILE)
}

#[test]
fn read_config_yaml_and_verify_using_index_access() {
    // Setup

    const TEST_VALUE_1: &str = "Test1";

    const TEST_VALUE_2: &str = "Test2";

    const TEST_VALUE_3: i32 = 2137;

    const TEST_VALUE_4: f32 = 2.137;

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure>::new(TEST_FILE2.to_string(), Box::new(yaml_handler));

    let test_structure_1 = TestStructure {
        test_value_1: TEST_VALUE_1.to_string(),
        test_value_2: TEST_VALUE_2.to_string(),
        test_structure_2: TestStructure2 {
            test_value_3: TEST_VALUE_3,
            test_structure_3: TestStructure3 {
                test_value_4: TEST_VALUE_4,
            },
        },
    };

    create_test_file(test_structure_1, TEST_FILE2);

    // Act
    let configuration_entry_result = configurable.read_config();

    // Verify

    match configuration_entry_result {
        Ok(configuration_entry) => {
            assert_eq!(
                get_configuration_entry_from_flat_key(&configuration_entry, "test_value_1")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                TEST_VALUE_1
            );
            assert_eq!(
                get_configuration_entry_from_flat_key(&configuration_entry, "test_value_2")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                TEST_VALUE_2
            );
            assert_eq!(
                *get_configuration_entry_from_flat_key(&configuration_entry, "test_structure_2.test_value_3")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                TEST_VALUE_3.to_string()
            );
            assert_eq!(
                *get_configuration_entry_from_flat_key(
                    &configuration_entry,
                    "test_structure_2.test_structure_3.test_value_4"
                )
                .unwrap()
                .get_value()
                .unwrap(),
                TEST_VALUE_4.to_string()
            );
        }
        Err(error) => {
            panic!("{}", error);
        }
    }
    remove_test_file(TEST_FILE2)
}

fn create_test_file(test_structure: TestStructure, file: &str) {
    let path = Path::new(file);
    if path.exists() {
        return;
    }

    let file = OpenOptions::new().create_new(true).append(true).open(file).unwrap();

    serde_yml::to_writer(file, &test_structure).unwrap();
}

fn remove_test_file(file: &str) {
    fs::remove_file(file).unwrap();
}
