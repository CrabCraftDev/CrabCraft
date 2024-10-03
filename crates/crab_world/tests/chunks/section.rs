use crate::init_registries;
use crab_world::blocks::Block;
use crab_world::chunks::section::ChunkSection;

#[test]
fn test_automatic_count_air_blocks() {
    init_registries();

    let mut section = ChunkSection::default();
    for x in 0..16 {
        for z in 0..16 {
            section.set_block(x, 0, z, Block(1)).unwrap()
        }
    }

    assert_eq!(section.non_air_block_amount, 16 * 16);
}

#[test]
fn test_count_air_blocks() {
    init_registries();

    let mut section = ChunkSection::default();
    for x in 0..5 {
        section.set_block(x, 0, 0, Block(1)).unwrap()
    }

    // Check if recalculation of non-air blocks is correct
    section.count_non_air_blocks();

    assert_eq!(section.non_air_block_amount, 5);
}

#[test]
fn modify_blocks() {
    init_registries();

    let mut section = ChunkSection::default();
    section.set_block(0, 0, 0, Block(5)).unwrap();

    assert_eq!(section.get_block(0, 0, 0), Some(Block(5)));
    assert_eq!(section.get_block(16, 0, 0), None);
}

#[test]
#[should_panic]
fn modify_invalid_block() {
    init_registries();

    let mut section = ChunkSection::default();
    section.set_block(0, 21, 0, Block(5)).unwrap();
}
