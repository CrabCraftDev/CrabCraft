use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::item_stack::ItemStack;
use crate::types::varint::VarInt;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x2E, state = Play)]
pub struct MerchantOffersS2CPacket {
    #[protocol_attribute(protocol_type = VarInt)]
    pub window_id: i32,
    pub trades: Vec<Trade>,
    #[protocol_attribute(protocol_type = VarInt)]
    pub villager_level: i32,
    #[protocol_attribute(protocol_type = VarInt)]
    pub villager_experience: i32,
    pub is_regular_villager: bool,
    pub can_restock: bool,
}

#[derive(Debug, Clone, PartialEq, ProtocolSerializer)]
pub struct Trade {
    pub first_input_item: Option<ItemStack>,
    pub output_item: Option<ItemStack>,
    pub second_input_item: Option<ItemStack>,
    pub trade_disabled: bool,
    pub number_of_uses: i32,
    pub max_number_of_uses: i32,
    pub experience: i32,
    pub special_price: i32,
    pub price_multiplier: f32,
    pub demand: i32,
}
