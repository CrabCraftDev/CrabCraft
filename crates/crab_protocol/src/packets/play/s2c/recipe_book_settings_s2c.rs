use crab_macros::{Packet, ProtocolSerializer};

use crate::protocol_state::ProtocolState::*;
use crate::types::recipe_book::RecipeBook;

#[derive(Packet, ProtocolSerializer, Debug, Clone)]
#[packet(name = "recipe_book_settings", id = 0x46, state = Play)]
pub struct RecipeBookSettingsS2CPacket {
    pub book: RecipeBook,
    pub is_opened: bool,
    pub is_filter_active: bool,
}
