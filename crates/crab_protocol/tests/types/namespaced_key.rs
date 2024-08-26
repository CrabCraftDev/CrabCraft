use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::types::namespaced_key::{NamespacedKey, MINECRAFT_NAMESPACE};

#[test]
fn test_encode_namespaced_key() {
    let id = NamespacedKey::new("crab", "craft").unwrap();
    let mut buf = vec![];

    id.serialize(&mut buf).unwrap();

    assert_eq!(buf, b"\x0acrab:craft")
}

#[test]
fn test_decode_namespaced_key() {
    let mut buf: &[u8] = b"\x0acrab:craft";
    let id = NamespacedKey::deserialize(&mut buf).unwrap();

    assert_eq!(id.namespace, "crab");
    assert_eq!(id.value, "craft");
}

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
