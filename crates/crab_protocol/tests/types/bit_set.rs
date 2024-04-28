
use crab_protocol::types::bit_set::BitSet;
#[test]
fn long_cycle() {
    let bit_set = BitSet::new([true, false].repeat(32));
    let new_bit_set = BitSet::from_long_array(&bit_set.as_long_array());
    assert_eq!(new_bit_set, bit_set);
}

#[test]
fn bytes_cycle() {
    let bit_set = BitSet::new([true, false].repeat(32));
    let new_bit_set = BitSet::from_bytes(&bit_set.as_bytes());
    assert_eq!(new_bit_set, bit_set);
}
