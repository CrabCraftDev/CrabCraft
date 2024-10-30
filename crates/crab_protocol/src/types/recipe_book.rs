use crab_macros::ProtocolSerializer;
use strum::FromRepr;

#[derive(Debug, Copy, Clone, PartialEq, ProtocolSerializer, FromRepr)]
pub enum RecipeBook {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}
