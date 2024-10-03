use crate::init_registries;
use crab_world::blocks::Block;
use crab_world::chunks::section::ChunkSection;

fn test_chunk_section_resizing(amount_of_init_blocks: u8, amount_of_test_blocks: u16) {
    init_registries();

    let mut section = ChunkSection::default();
    for i in 0..amount_of_init_blocks {
        section.set_block(i, i + 1, i, Block(i as u16)).unwrap();
    }

    let mut test_section = section.clone();
    for b in 2137..(2137 + amount_of_test_blocks) {
        test_section.set_block(0, 0, 0, Block(b)).unwrap();
    }
    test_section.set_block(0, 0, 0, Block(0)).unwrap();
    test_section.blocks.shrink_to_fit().unwrap();

    assert_eq!(section, test_section);
}

#[test]
fn resize_indirect_direct() {
    test_chunk_section_resizing(5, 400);
}

#[test]
fn resize_indirect() {
    test_chunk_section_resizing(5, 100);
}

#[test]
fn resize_single() {
    test_chunk_section_resizing(0, 400);
}
