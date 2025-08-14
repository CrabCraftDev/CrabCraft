use crate::utils::{TestConfig, TestEnum};
use crab_config::SemverVersion;
use crab_config::configurable::Configurable;
use crab_config::handlers::yaml_handler::YamlHandler;
use crab_config::node::ConfigurationNode;
use crab_config::types::config::Config;
use crab_config::version_control::migrations::migration::Migration;
use semver::Version;
use serde_yml::{Number, Value};
use std::fs;

const TEST_FILE: &str = "migration_config_test.yml";
const TEST_FILE_MULTIPLE: &str = "migration_config_test_multiple.yml";

struct FirstMigration;

impl Migration<Version> for FirstMigration {
    fn up(&self, configuration_node: &mut ConfigurationNode) {
        configuration_node.set_value_by_flat_path("test", Value::Number(Number::from(69420)));
    }

    fn version(&self) -> Version {
        Version::new(0, 0, 1)
    }
}

struct SecondMigration;

impl Migration<Version> for SecondMigration {
    fn up(&self, configuration_node: &mut ConfigurationNode) {
        configuration_node.set_value_by_flat_path("test", Value::Number(Number::from(420)));
    }

    fn version(&self) -> Version {
        Version::new(0, 0, 2)
    }
}

#[test]
fn migration_test() {
    let configurable = Configurable::<TestConfig, Version>::new(TEST_FILE.to_string(), Box::new(YamlHandler {}));
    configurable.load_or_create_default_config(Vec::new()).unwrap();

    configurable
        .write_node(
            TestConfig {
                _version: SemverVersion::new(0, 0, 0).to_string(),
                test_enum: TestEnum::T2,
                test: 0,
            }
            .as_configuration_node(),
        )
        .unwrap();

    let migration_list: Vec<Box<dyn Migration<Version>>> = vec![Box::new(FirstMigration)];
    let test_config = configurable.read_config_and_update(migration_list).unwrap();

    assert_eq!(test_config.test, 69420);
    assert_eq!(test_config.test_enum, TestEnum::T2);
    assert_eq!(test_config._version, "0.0.2");
    fs::remove_file(TEST_FILE).unwrap();
}

#[test]
fn multiple_migrations_test() {
    let configurable =
        Configurable::<TestConfig, Version>::new(TEST_FILE_MULTIPLE.to_string(), Box::new(YamlHandler {}));
    configurable.load_or_create_default_config(Vec::new()).unwrap();

    configurable
        .write_node(
            TestConfig {
                _version: SemverVersion::new(0, 0, 0).to_string(),
                test_enum: TestEnum::T2,
                test: 0,
            }
            .as_configuration_node(),
        )
        .unwrap();

    let migration_list: Vec<Box<dyn Migration<Version>>> = vec![Box::new(FirstMigration), Box::new(SecondMigration)];

    let test_config = configurable.read_config_and_update(migration_list).unwrap();

    assert_eq!(test_config.test, 420);
    assert_eq!(test_config.test_enum, TestEnum::T2);
    assert_eq!(test_config._version, "0.0.2");

    fs::remove_file(TEST_FILE_MULTIPLE).unwrap();
}
