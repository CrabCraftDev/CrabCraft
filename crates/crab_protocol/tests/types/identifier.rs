use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::identifier::Identifier;

#[test]
fn test_encode_identifier() {
    let id = Identifier::new("crab", "craft").unwrap();
    let mut buf = vec![];

    id.serialize(&mut buf).unwrap();

    assert_eq!(buf, b"\x0acrab:craft")
}

#[test]
fn test_decode_identifier() {
    let mut buf: &[u8] = b"\x0acrab:craft";
    let id = Identifier::deserialize(&mut buf).unwrap();

    assert_eq!(id.namespace, "crab");
    assert_eq!(id.value, "craft");
}

#[test]
fn test_to_string_mc_identifier() {
    let id = Identifier::minecraft("crabcraft").unwrap();

    assert_eq!(id.namespace, "minecraft");
    assert_eq!(id.value, "crabcraft");
    assert_eq!(id.to_string(), "minecraft:crabcraft")
}

#[test]
fn test_identifier_validator() {
    assert!(Identifier::new("CrabCraft", "test").is_err());
    assert!(Identifier::minecraft("path/to/something").is_ok());
    assert!(Identifier::minecraft("path/To/Error").is_err())
}
