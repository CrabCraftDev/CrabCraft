use bytes::BufMut;

use crab_protocol::protocol_serializer::ProtocolSerializer;
use crab_protocol::types::block_position::BlockPosition;

#[test]
fn test_encode_position() {
    let mut buf = Vec::new();
    buf.put_i64(0b0100011000000111011000110010110000010101101101001000001100111111);

    let mut encoded = Vec::new();
    let block_pos = BlockPosition {
        x: 18357644,
        y: 831,
        z: -20882616,
    };
    block_pos.serialize(&mut encoded).unwrap();
    assert_eq!(encoded, buf)
}

#[test]
fn test_decode_position() {
    let mut buf = Vec::new();
    buf.put_i64(0b0100011000000111011000110010110000010101101101001000001100111111);

    let block_pos = BlockPosition::deserialize(&mut buf.as_slice()).unwrap();
    assert_eq!(18357644, block_pos.x);
    assert_eq!(831, block_pos.y);
    assert_eq!(-20882616, block_pos.z);
}

#[test]
fn test_position_validation() {
    BlockPosition::new(18357644, 831, -20882616).unwrap();
    BlockPosition::new(18357644, 831, 33554435).expect_err("Should fail Z");
    BlockPosition::new(18357644, -4096, -20882616).expect_err("Should fail Y");
}
