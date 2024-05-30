use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::protocol::ProtocolState::*;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(id = 0x28, state = Play)]
pub struct ChangeRecipeBookSettingsC2SPacket {
    pub book: RecipeBook,
    pub is_opened: bool,
    pub is_filter_active: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum RecipeBook {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}
