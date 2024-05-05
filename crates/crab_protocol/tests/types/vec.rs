use bytes::BytesMut;

use crab_protocol::protocol_serializer::ProtocolSerializer;

#[test]
fn test_vec_protocol_type() {
    let original_vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    let mut buf = BytesMut::new();
    original_vec.serialize(&mut buf).unwrap();

    let deserialized_vec = Vec::<u8>::deserialize(&mut buf.freeze()).unwrap();

    assert_eq!(original_vec, deserialized_vec);
}
