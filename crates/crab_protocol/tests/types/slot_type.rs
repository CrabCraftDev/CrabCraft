use bytes::BytesMut;
use crab_nbt::Nbt;
use crab_protocol::packets::play::{Equipment, EquipmentSlot};
use crab_protocol::protocol_type::ProtocolType;
use crab_protocol::types::item_stack::ItemStack;
use std::collections::HashMap;

#[test]
fn cycle_equipment() {
    let equipment = Equipment {
        slots: HashMap::from([
            (
                EquipmentSlot::MainHand,
                Some(ItemStack::new(1, 1, Nbt::default()).unwrap()),
            ),
            (
                EquipmentSlot::OffHand,
                Some(ItemStack::new(2, 1, Nbt::default()).unwrap()),
            ),
            (
                EquipmentSlot::Boots,
                Some(ItemStack::new(3, 1, Nbt::default()).unwrap()),
            ),
            (
                EquipmentSlot::Legging,
                Some(ItemStack::new(4, 1, Nbt::default()).unwrap()),
            ),
        ]),
    };
    let mut buf = BytesMut::new();
    equipment.serialize(&mut buf).unwrap();
    let deserialized = Equipment::deserialize(&mut buf).unwrap();
    assert_eq!(equipment.slots, deserialized.slots);
}
