use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::recipe_book::RecipeBook;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "player_input", id = 0x28, state = Play)]
pub struct ChangeRecipeBookSettingsC2SPacket {
    pub book: RecipeBook,
    pub is_opened: bool,
    pub is_filter_active: bool,
}
