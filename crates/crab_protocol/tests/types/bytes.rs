use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::bytes::FixedBytes;

#[test]
fn bytes_serialize_correct() {
    let test: FixedBytes<10> = FixedBytes("0123456789".into());
    let mut encoded = Vec::new();
    test.serialize(&mut encoded).unwrap();
    assert_eq!(encoded.len(), 10);
}

#[test]
fn bytes_serialize_invalid_size() {
    let test: FixedBytes<10> = FixedBytes("0".into());
    let mut encoded = Vec::new();
    test.serialize(&mut encoded).expect_err("Should fail");
}
