use crate::utils::{create_test_file, remove_test_file};
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::node::search::get_configuration_node_from_flat_key;
use crab_macros::Config;
use semver::Version;
use serde::Serialize;
use serde_yml::{Number, Value};

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
    const TEST_VALUE_4: f64 = 2.137;

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure, Version>::new(TEST_FILE.to_string(), Box::new(yaml_handler));

    let test_structure_1 = TestStructure {
        test_value_1: TEST_VALUE_1.to_string(),
        test_value_2: TEST_VALUE_2.to_string(),
        test_structure_2: TestStructure2 {
            test_value_3: TEST_VALUE_3,
            test_structure_3: TestStructure3 {
                test_value_4: TEST_VALUE_4 as f32,
            },
        },
    };

    create_test_file(test_structure_1, TEST_FILE);

    // Act
    let configuration_node = configurable.read_config().unwrap();

    // Verify

    let map = configuration_node.serialize_to_flatten_map_with_path().unwrap();
    assert_eq!(map.get("test_value_1").unwrap(), TEST_VALUE_1);
    assert_eq!(map.get("test_value_2").unwrap(), TEST_VALUE_2);
    assert_eq!(
        map.get("test_structure_2.test_value_3").unwrap(),
        &Value::Number(Number::from(TEST_VALUE_3))
    );
    assert_eq!(
        map.get("test_structure_2.test_structure_3.test_value_4").unwrap(),
        &Value::Number(Number::from(TEST_VALUE_4))
    );

    remove_test_file(TEST_FILE)
}

#[test]
fn read_config_yaml_and_verify_using_index_access() {
    // Setup
    const TEST_VALUE_1: &str = "Test1";
    const TEST_VALUE_2: &str = "Test2";
    const TEST_VALUE_3: i32 = 2137;
    const TEST_VALUE_4: f64 = 2.137;

    let yaml_handler: YamlHandler = YamlHandler {};
    let configurable = Configurable::<TestStructure, Version>::new(TEST_FILE2.to_string(), Box::new(yaml_handler));

    let test_structure_1 = TestStructure {
        test_value_1: TEST_VALUE_1.to_string(),
        test_value_2: TEST_VALUE_2.to_string(),
        test_structure_2: TestStructure2 {
            test_value_3: TEST_VALUE_3,
            test_structure_3: TestStructure3 {
                test_value_4: TEST_VALUE_4 as f32,
            },
        },
    };

    create_test_file(test_structure_1, TEST_FILE2);

    // Act
    let configuration_node_result = configurable.read_config();

    // Verify
    match configuration_node_result {
        Ok(configuration_node) => {
            assert_eq!(
                get_configuration_node_from_flat_key(&configuration_node, "test_value_1")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                TEST_VALUE_1
            );
            assert_eq!(
                get_configuration_node_from_flat_key(&configuration_node, "test_value_2")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                TEST_VALUE_2
            );
            assert_eq!(
                *get_configuration_node_from_flat_key(&configuration_node, "test_structure_2.test_value_3")
                    .unwrap()
                    .get_value()
                    .unwrap(),
                Value::Number(Number::from(TEST_VALUE_3))
            );
            assert_eq!(
                *get_configuration_node_from_flat_key(
                    &configuration_node,
                    "test_structure_2.test_structure_3.test_value_4"
                )
                .unwrap()
                .get_value()
                .unwrap(),
                Value::from(Number::from(TEST_VALUE_4))
            );
        }
        Err(error) => {
            panic!("{}", error);
        }
    }
    remove_test_file(TEST_FILE2)
}
