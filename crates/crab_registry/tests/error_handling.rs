use crab_registry::error::RegistryError;
use crab_registry::key::RegistryKey;
use crab_registry::minecraft::Biome;
use crab_registry::registry::{RegistryBase, SimpleRegistry, TypedRegistryBase};
use crab_registry::{REGISTRY, Registrable};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;

#[test]
fn test_try_value_by_key_error() {
    let root = REGISTRY.load();

    let nonexistent_key = NamespacedKey::from_str("minecraft:nonexistent_biome").unwrap();
    let result = root.try_value_by_key::<Biome>(&nonexistent_key);

    assert!(result.is_err());
    if let Err(RegistryError::MissingKeyInRegistry { key, registry }) = result {
        assert_eq!(key.to_string(), "minecraft:nonexistent_biome");
        assert_eq!(registry, "minecraft:worldgen/biome");
    } else {
        panic!("Expected MissingKeyInRegistry error");
    }
}

#[test]
fn test_try_id_to_key_success() {
    let root = REGISTRY.load();

    let result = root.try_id_to_key::<Biome>(0);

    assert!(result.is_ok());
    let key = result.unwrap();
    assert!(key.to_string().starts_with("minecraft:"));
}

#[test]
#[should_panic]
fn test_nonexistent_registry_key_panic() {
    #[derive(Debug)]
    struct TestType {}

    impl Registrable for TestType {
        type RegistryType = SimpleRegistry<Self>;
        const REGISTRY: RegistryKey<Self> = RegistryKey::new("test:type");
    }

    impl TestType {
        const NONEXISTENT: RegistryKey<Self> = RegistryKey::new("test:nonexistent");
    }

    // This should panic because the registry doesn't exist or the key doesn't exist
    let _id = TestType::NONEXISTENT.to_numeric_id();
}

#[test]
fn test_empty_registry_operations() {
    let registry = SimpleRegistry::<Biome>::default();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);

    let test_key = NamespacedKey::from_str("minecraft:plains").unwrap();

    assert!(registry.value_by_key(&test_key).is_none());
    assert!(registry.key_to_id(&test_key).is_none());
    assert!(registry.id_to_key(0).is_none());
    assert!(registry.value_by_id(0).is_none());
}

#[test]
fn test_multiple_errors_in_sequence() {
    let root = REGISTRY.load();

    // Test multiple non-existent keys
    let keys = vec![
        "minecraft:nonexistent1",
        "minecraft:nonexistent2",
        "test:invalid",
        "foo:bar",
    ];

    for key_str in keys {
        let key = NamespacedKey::from_str(key_str).unwrap();
        let result = root.try_key_to_id::<Biome>(&key);
        assert!(result.is_err());
    }

    // Test multiple invalid IDs
    let biome_registry = root.registry_by_type::<Biome>();
    let len = biome_registry.len();

    for offset in 1..10 {
        let invalid_id = len + offset;
        let result = root.try_id_to_key::<Biome>(invalid_id);
        assert!(result.is_err());
    }
}
