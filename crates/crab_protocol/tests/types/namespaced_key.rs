use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_text::utils::namespaced_key::NamespacedKey;

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
