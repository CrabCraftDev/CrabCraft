use bytes::BytesMut;
use crab_nbt::Nbt;

use crab_protocol::protocol_serializer::ProtocolSerializer;

#[test]
fn nbt_serialization_deserialization() {
    let nbt = Nbt::default();
    let mut buf = BytesMut::new();
    nbt.serialize(&mut buf).unwrap();
    let deserialized = Nbt::deserialize(&mut buf).unwrap();
    assert_eq!(nbt, deserialized);
}
