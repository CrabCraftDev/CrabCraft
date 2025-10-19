use crab_registry::key::RegistryKey;
use crab_registry::registry::{RegistryBase, RootRegistry, SimpleRegistry, TypedRegistryBase};
use crab_registry::{REGISTRY, Registrable};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct TestEntity {
    pub name: String,
    pub level: u32,
}

impl Registrable for TestEntity {
    type RegistryType = SimpleRegistry<Self>;

    const REGISTRY: RegistryKey<Self> = RegistryKey::new("test:entities");
}

#[test]
fn test_custom_registry_creation() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();
    let mage_key = NamespacedKey::from_str("test:mage").unwrap();

    registry.register(
        warrior_key.clone(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );
    registry.register(
        mage_key.clone(),
        TestEntity {
            name: "Mage".to_string(),
            level: 8,
        },
    );

    assert_eq!(registry.len(), 2);

    let warrior = registry.value_by_key(&warrior_key).unwrap();
    assert_eq!(warrior.name, "Warrior");
    assert_eq!(warrior.level, 10);

    let mage = registry.value_by_key(&mage_key).unwrap();
    assert_eq!(mage.name, "Mage");
    assert_eq!(mage.level, 8);
}

#[test]
fn test_custom_registry_in_root() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    registry.register(
        NamespacedKey::from_str("test:warrior").unwrap(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );

    let mut root = RootRegistry::default();
    root.register_registry(TestEntity::REGISTRY, registry);

    let entity_registry = root.registry_by_type::<TestEntity>();
    assert_eq!(entity_registry.len(), 1);

    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();
    let warrior = entity_registry.value_by_key(&warrior_key).unwrap();
    assert_eq!(warrior.name, "Warrior");
}

#[test]
fn test_registry_value_sharing() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();
    registry.register(
        warrior_key.clone(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );

    // Get value by key
    let value1 = registry.value_by_key(&warrior_key).unwrap();

    // Get value by id
    let warrior_id = registry.key_to_id(&warrior_key).unwrap();
    let value2 = registry.value_by_id(warrior_id).unwrap();

    // Both should point to the same Arc
    assert!(Arc::ptr_eq(value1, value2));
}

#[test]
fn test_custom_registry_insertion_order() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    let keys = [
        NamespacedKey::from_str("test:first").unwrap(),
        NamespacedKey::from_str("test:second").unwrap(),
        NamespacedKey::from_str("test:third").unwrap(),
        NamespacedKey::from_str("test:fourth").unwrap(),
    ];

    for (i, key) in keys.iter().enumerate() {
        registry.register(
            key.clone(),
            TestEntity {
                name: format!("Entity {}", i),
                level: i as u32,
            },
        );
    }

    // Verify insertion order is preserved
    for (expected_id, key) in keys.iter().enumerate() {
        let actual_id = registry.key_to_id(key).unwrap();
        assert_eq!(expected_id, actual_id, "Registration order should be preserved");
    }

    // Verify we can iterate in order
    for (id, expected_key) in keys.iter().enumerate() {
        let key = registry.id_to_key(id).unwrap();
        assert_eq!(key, expected_key, "Iteration order should be preserved");
    }
}

#[test]
fn test_custom_registry_id_conversions() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();
    let mage_key = NamespacedKey::from_str("test:mage").unwrap();

    registry.register(
        warrior_key.clone(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );
    registry.register(
        mage_key.clone(),
        TestEntity {
            name: "Mage".to_string(),
            level: 8,
        },
    );

    // Test warrior conversions
    let warrior_id = registry.key_to_id(&warrior_key).unwrap();
    let recovered_warrior_key = registry.id_to_key(warrior_id).unwrap();
    assert_eq!(&warrior_key, recovered_warrior_key);

    // Test mage conversions
    let mage_id = registry.key_to_id(&mage_key).unwrap();
    let recovered_mage_key = registry.id_to_key(mage_id).unwrap();
    assert_eq!(&mage_key, recovered_mage_key);

    // IDs should be different
    assert_ne!(warrior_id, mage_id);
}

#[test]
fn test_custom_registry_missing_keys() {
    let registry = SimpleRegistry::<TestEntity>::default();

    let nonexistent_key = NamespacedKey::from_str("test:nonexistent").unwrap();

    assert!(registry.value_by_key(&nonexistent_key).is_none());
    assert!(registry.key_to_id(&nonexistent_key).is_none());
}

#[test]
fn test_custom_registry_invalid_ids() {
    let mut registry = SimpleRegistry::<TestEntity>::default();

    registry.register(
        NamespacedKey::from_str("test:warrior").unwrap(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );

    // Registry has 1 entry (id 0), so other ids should be invalid
    assert!(registry.id_to_key(1).is_none());
    assert!(registry.value_by_id(1).is_none());
    assert!(registry.id_to_key(100).is_none());
    assert!(registry.value_by_id(100).is_none());
}

#[test]
fn test_root_registry_helpers_with_custom_types() {
    let mut custom_registry = SimpleRegistry::<TestEntity>::default();
    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();

    custom_registry.register(
        warrior_key.clone(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );

    let mut root = RootRegistry::default();
    root.register_registry(TestEntity::REGISTRY, custom_registry);

    // Test key_to_id helper
    let warrior_id = root.key_to_id::<TestEntity>(&warrior_key).unwrap();
    assert_eq!(warrior_id, 0);

    // Test id_to_key helper
    let recovered_key = root.id_to_key::<TestEntity>(warrior_id).unwrap();
    assert_eq!(&warrior_key, recovered_key);

    // Test value_by_key helper
    let warrior_value = root.value_by_key::<TestEntity>(&warrior_key).unwrap();
    assert_eq!(warrior_value.name, "Warrior");
    assert_eq!(warrior_value.level, 10);
}

#[test]
fn test_custom_registry_in_global() {
    let mut custom_registry = SimpleRegistry::<TestEntity>::default();
    custom_registry.register(
        NamespacedKey::from_str("test:warrior").unwrap(),
        TestEntity {
            name: "Warrior".to_string(),
            level: 10,
        },
    );

    let mut root = RootRegistry::populated().unwrap();
    root.register_registry(TestEntity::REGISTRY, custom_registry);

    // Store in global registry
    REGISTRY.store(Arc::new(root));

    // Load and verify
    let loaded_root = REGISTRY.load();
    let entity_registry = loaded_root.registry_by_type::<TestEntity>();

    let warrior_key = NamespacedKey::from_str("test:warrior").unwrap();
    let warrior = entity_registry.value_by_key(&warrior_key).unwrap();
    assert_eq!(warrior.name, "Warrior");

    // Restore original state
    REGISTRY.store(Arc::new(RootRegistry::populated().unwrap()));
}
