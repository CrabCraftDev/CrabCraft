use std::collections::HashMap;

use derive_more::Into;
use strum::FromRepr;

use crab_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol_serializer::ProtocolSerializer;
use crate::protocol_state::ProtocolState::*;
use crate::types::item_stack::ItemStack;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x5B, state = Play)]
pub struct SetEquipmentS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub entity_id: i32,
    pub equipment: Equipment,
}

#[derive(Clone, Default, Debug)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, Option<ItemStack>>,
}

impl ProtocolSerializer for Equipment {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let slots_len = self.slots.len();
        for (index, (slot, item)) in self.slots.iter().enumerate() {
            let mut slot = *slot as u8;
            if index != slots_len - 1 {
                slot |= 128;
            }
            buf.put_u8(slot);
            ProtocolSerializer::serialize(item, buf)?;
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let mut slots = HashMap::new();
        loop {
            let slot_byte = buf.get_u8();
            let slot_enum_index = (slot_byte & 127) as usize;

            let slot = EquipmentSlot::from_repr(slot_enum_index)
                .ok_or(ProtocolError::EnumOutOfBounds("Equipment".to_string(), slot_enum_index))?;
            let item = ProtocolSerializer::deserialize(buf)?;
            slots.insert(slot, item);

            if slot_byte & 128 == 0 {
                break;
            }
        }
        Ok(Self { slots })
    }
}

#[derive(ProtocolSerializer, FromRepr, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}
