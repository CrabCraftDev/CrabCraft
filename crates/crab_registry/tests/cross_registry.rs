use crab_registry::REGISTRY;
use crab_registry::minecraft::*;
use crab_registry::registry::{RegistryBase, TypedRegistryBase};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;

#[test]
fn test_vanilla_registries_consistency() {
    let root = REGISTRY.load();
    let all_registries = root.all_registries();

    for (name, registry) in all_registries {
        let len = registry.len();
        assert!(len > 0, "Registry '{}' should not be empty", name);

        // Verify all IDs are sequential from 0 to len-1
        for id in 0..len {
            let key = registry
                .id_to_key(id)
                .unwrap_or_else(|| panic!("Registry '{}' should have ID {}", name, id));

            // Verify key maps back to same ID
            let recovered_id = registry
                .key_to_id(key)
                .unwrap_or_else(|| panic!("Registry '{}' key {:?} should have an ID", name, key));

            assert_eq!(id, recovered_id, "Registry '{}' ID mismatch for key {:?}", name, key);
        }

        // Verify ID beyond range returns None
        assert!(
            registry.id_to_key(len).is_none(),
            "Registry '{}' should not have ID {}",
            name,
            len
        );
    }
}

#[test]
fn test_sequential_registry_access() {
    let root = REGISTRY.load();

    // Access different registries one after another
    let biome = {
        let registry = root.registry_by_type::<Biome>();
        let key = NamespacedKey::from_str("minecraft:plains").unwrap();
        registry.value_by_key(&key).cloned()
    };
    assert!(biome.is_some());

    let damage_type = {
        let registry = root.registry_by_type::<DamageType>();
        let first_key = registry.id_to_key(0).unwrap();
        registry.value_by_key(first_key).cloned()
    };
    assert!(damage_type.is_some());

    let cat_variant = {
        let registry = root.registry_by_type::<CatVariant>();
        let first_key = registry.id_to_key(0).unwrap();
        registry.value_by_key(first_key).cloned()
    };
    assert!(cat_variant.is_some());
}

#[test]
fn test_iteration_order_consistency() {
    let root = REGISTRY.load();
    let biome_registry = root.registry_by_type::<Biome>();

    // Iterate twice and verify order is the same
    let first_pass: Vec<String> = (0..biome_registry.len())
        .map(|id| biome_registry.id_to_key(id).unwrap().to_string())
        .collect();

    let second_pass: Vec<String> = (0..biome_registry.len())
        .map(|id| biome_registry.id_to_key(id).unwrap().to_string())
        .collect();

    assert_eq!(first_pass, second_pass, "Iteration order should be consistent");
}

/// Test that root registry correctly dispatches to typed registries
#[test]
fn test_root_registry_dispatch() {
    let root = REGISTRY.load();

    let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();

    // Access through root registry helper
    let via_root = root.value_by_key::<Biome>(&plains_key);

    // Access through typed registry
    let biome_registry = root.registry_by_type::<Biome>();
    let via_typed = biome_registry.value_by_key(&plains_key);

    // Both should return the same value
    assert!(via_root.is_some());
    assert!(via_typed.is_some());
    assert!(std::sync::Arc::ptr_eq(via_root.unwrap(), via_typed.unwrap()));
}
