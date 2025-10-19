use crab_registry::REGISTRY;
use crab_registry::minecraft::*;
use crab_registry::registry::{RegistryBase, TypedRegistryBase};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;

// Some smoke tests to verify that vanilla registries are populated and accessible.
// These tests are not exhaustive and may not cover all cases.

#[test]
fn test_vanilla_registries_populated() {
    let root = REGISTRY.load();

    // Verify that registries exist and contain entries
    let biome_registry = root.registry_by_type::<Biome>();
    assert!(!biome_registry.is_empty(), "Biome registry should not be empty");
    assert!(biome_registry.len() > 0);

    let damage_type_registry = root.registry_by_type::<DamageType>();
    assert!(
        !damage_type_registry.is_empty(),
        "DamageType registry should not be empty"
    );

    let dimension_type_registry = root.registry_by_type::<DimensionType>();
    assert!(
        !dimension_type_registry.is_empty(),
        "DimensionType registry should not be empty"
    );

    let cat_variant_registry = root.registry_by_type::<CatVariant>();
    assert!(
        !cat_variant_registry.is_empty(),
        "CatVariant registry should not be empty"
    );
}

#[test]
fn test_vanilla_known_entries_exist() {
    let root = REGISTRY.load();
    let biome_registry = root.registry_by_type::<Biome>();

    // Test that common biomes exist
    let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();
    assert!(
        biome_registry.value_by_key(&plains_key).is_some(),
        "Plains biome should exist"
    );

    let void_key = NamespacedKey::from_str("minecraft:the_void").unwrap();
    assert!(
        biome_registry.value_by_key(&void_key).is_some(),
        "The Void biome should exist"
    );
}

#[test]
fn test_vanilla_id_conversion() {
    let root = REGISTRY.load();
    let biome_registry = root.registry_by_type::<Biome>();

    let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();

    let plains_id = biome_registry.key_to_id(&plains_key).expect("Plains should have an ID");
    let recovered_key = biome_registry.id_to_key(plains_id).expect("ID should map back to key");

    assert_eq!(
        &plains_key, recovered_key,
        "Key should round-trip through ID conversion"
    );
}

#[test]
fn test_multiple_vanilla_registries_independence() {
    let root = REGISTRY.load();

    let biome_registry = root.registry_by_type::<Biome>();
    let damage_type_registry = root.registry_by_type::<DamageType>();
    let painting_variant_registry = root.registry_by_type::<PaintingVariant>();

    let biome_count = biome_registry.len();
    let damage_type_count = damage_type_registry.len();
    let painting_count = painting_variant_registry.len();

    assert!(biome_count > 0);
    assert!(damage_type_count > 0);
    assert!(painting_count > 0);

    // Note: This is a weak test but checks basic independence
    assert!(
        biome_count != damage_type_count || damage_type_count != painting_count,
        "Registries should have different sizes (independence check)"
    );
}

#[test]
fn test_registry_key_to_numeric_id() {
    let plains_id = Biome::PLAINS.to_numeric_id();
    let void_id = Biome::THE_VOID.to_numeric_id();

    // IDs should be different
    assert_ne!(plains_id, void_id, "Different biomes should have different numeric IDs");

    // IDs should be stable across calls
    assert_eq!(Biome::PLAINS.to_numeric_id(), plains_id);
    assert_eq!(Biome::THE_VOID.to_numeric_id(), void_id);
}

#[test]
fn test_root_registry_conversion_helpers() {
    let root = REGISTRY.load();

    let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();

    // Use root registry helpers
    let plains_id = root.key_to_id::<Biome>(&plains_key).expect("Plains should exist");
    let recovered_key = root.id_to_key::<Biome>(plains_id).expect("ID should be valid");

    assert_eq!(&plains_key, recovered_key);
}

#[test]
fn test_registry_lengths() {
    let root = REGISTRY.load();

    // Biomes should have many entries (there are 60+ vanilla biomes)
    let biome_registry = root.registry_by_type::<Biome>();
    assert!(
        biome_registry.len() >= 50,
        "Should have at least 50 biomes, got {}",
        biome_registry.len()
    );

    // Painting variants should have fewer entries (there are ~20 paintings)
    let painting_registry = root.registry_by_type::<PaintingVariant>();
    assert!(
        painting_registry.len() >= 10,
        "Should have at least 10 paintings, got {}",
        painting_registry.len()
    );
    assert!(
        painting_registry.len() < 100,
        "Should have fewer than 100 paintings, got {}",
        painting_registry.len()
    );

    // Dimension types should have very few entries (3 in vanilla: overworld, nether, end)
    let dimension_registry = root.registry_by_type::<DimensionType>();
    assert!(
        dimension_registry.len() >= 3,
        "Should have at least 3 dimensions, got {}",
        dimension_registry.len()
    );
    assert!(
        dimension_registry.len() < 20,
        "Should have fewer than 20 dimensions, got {}",
        dimension_registry.len()
    );
}
