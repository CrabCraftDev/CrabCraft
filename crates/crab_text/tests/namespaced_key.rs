use crab_text::utils::namespaced_key::{NamespacedKey, MINECRAFT_NAMESPACE};

#[test]
fn test_to_string_mc_namespaced_key() {
    let id = NamespacedKey::minecraft("crabcraft").unwrap();

    assert_eq!(id.namespace, MINECRAFT_NAMESPACE);
    assert_eq!(id.value, "crabcraft");
    assert_eq!(id.to_string(), "minecraft:crabcraft")
}

#[test]
fn test_namespaced_key_validator() {
    assert!(NamespacedKey::new("CrabCraft", "test").is_err());
    assert!(NamespacedKey::minecraft("path/to/something").is_ok());
    assert!(NamespacedKey::minecraft("path/To/Error").is_err())
}
