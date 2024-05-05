use strum::FromRepr;

use crab_protocol_macros::{Packet, ProtocolSerializer};

use crate::buf::{ReadByteBuf, WriteByteBuf};
use crate::error::ProtocolError;
use crate::protocol::ProtocolState::*;
use crate::protocol_serializer::ProtocolSerializer;
use crate::types::identifier::Identifier;

#[derive(Packet, Debug, Clone)]
#[packet(id = 0x3F, state = Play)]
pub struct UpdateRecipeBookS2CPacket {
    pub action: RecipeBookAction,
    pub settings: RecipeBookSettings,
    pub recipes: Vec<Identifier>,
}

impl ProtocolSerializer for UpdateRecipeBookS2CPacket {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_var_int(match self.action {
            RecipeBookAction::Init { .. } => 0,
            RecipeBookAction::Add => 1,
            RecipeBookAction::Remove => 2,
        });
        self.settings.serialize(buf)?;
        self.recipes.serialize(buf)?;
        if let RecipeBookAction::Init { to_highlight } = &self.action {
            to_highlight.serialize(buf)?;
        }
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let action_id = buf.get_var_int()?;
        let settings = RecipeBookSettings::deserialize(buf)?;
        let recipes = Vec::<Identifier>::deserialize(buf)?;
        let action = match action_id {
            0 => {
                let to_highlight = Vec::<Identifier>::deserialize(buf)?;
                RecipeBookAction::Init { to_highlight }
            }
            1 => RecipeBookAction::Add,
            2 => RecipeBookAction::Remove,
            _ => {
                return Err(ProtocolError::EnumOutOfBounds(
                    "RecipeBookAction".to_string(),
                    action_id as usize,
                ))
            }
        };
        Ok(Self {
            action,
            settings,
            recipes,
        })
    }
}

#[derive(Debug, Clone, PartialEq, FromRepr)]
#[repr(i32)]
pub enum RecipeBookAction {
    Init { to_highlight: Vec<Identifier> },
    Add,
    Remove,
}

#[derive(ProtocolSerializer, Clone, Debug)]
pub struct RecipeBookSettings {
    pub open_crafting_recipe_book: bool,
    pub filter_crafting_recipe_book: bool,
    pub open_smelting_recipe_book: bool,
    pub filter_smelting_recipe_book: bool,
    pub open_blast_furnace_recipe_book: bool,
    pub filter_blast_furnace_recipe_book: bool,
    pub open_smoker_recipe_book: bool,
    pub filter_smoker_recipe_book: bool,
}
