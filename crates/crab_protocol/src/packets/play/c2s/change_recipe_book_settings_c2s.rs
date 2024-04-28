use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolType};

use crate::protocol::ProtocolState::*;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x25, state = Play)]
pub struct ChangeRecipeBookSettingsC2SPacket {
    pub book: RecipeBook,
    pub is_opened: bool,
    pub is_filter_active: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolType, FromRepr)]
pub enum RecipeBook {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}
