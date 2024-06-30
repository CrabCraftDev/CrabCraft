use std::any::Any;

use crab_registry::blocks::{Bedrock, PrismarineSlab, PrismarineSlabKind};

#[test]
pub fn state_cycle_complex() {
    let block = PrismarineSlab {
        kind: PrismarineSlabKind::Bottom,
        waterlogged: false.into(),
    };

    let test_block: Box<dyn Any> = Box::new(block.clone());
    let test_block = test_block.downcast_ref::<PrismarineSlab>().unwrap();

    assert_eq!(&block, test_block);
}

#[test]
pub fn state_cycle_simple() {
    let block = Bedrock;

    let test_block: Box<dyn Any> = Box::new(block.clone());
    let test_block = test_block.downcast_ref::<Bedrock>().unwrap();

    assert_eq!(&block, test_block);
}
