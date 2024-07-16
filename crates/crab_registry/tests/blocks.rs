use std::any::Any;

use crab_registry::blocks::{Bedrock, Block, PrismarineSlab, PrismarineSlabKind};

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

#[test]
pub fn block_type_comparison() {
    let bedrock = Bedrock;
    let prismarine_slab = PrismarineSlab {
        kind: PrismarineSlabKind::Bottom,
        ..Default::default()
    };
    let prismarine_slab2 = PrismarineSlab {
        kind: PrismarineSlabKind::Top,
        ..Default::default()
    };
    let boxed_bedrock = Box::new(Bedrock);

    assert!(Bedrock::is_same_type_as_self(&bedrock));
    assert!(Bedrock::is_same_type_as_self(&*boxed_bedrock));
    assert!(!Bedrock::is_same_type_as_self(&prismarine_slab));
    assert!(PrismarineSlab::is_same_type_as_self(&prismarine_slab));
    assert!(!PrismarineSlab::is_same_type_as_self(&bedrock));

    assert!(prismarine_slab.is_same_type_as(&prismarine_slab2));
    assert!(prismarine_slab2.is_same_type_as(&prismarine_slab));
    assert!(bedrock.is_same_type_as(&*boxed_bedrock));
    assert!(boxed_bedrock.is_same_type_as(&bedrock));
    assert!(!bedrock.is_same_type_as(&prismarine_slab));
}
