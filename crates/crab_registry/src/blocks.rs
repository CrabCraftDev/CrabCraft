use std::{
    any::{Any, TypeId},
    fmt::Debug,
};
use strum::FromRepr;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));

pub trait Block: Debug + Any {
    /// Gets current Block state based on set fields. Used mostly in protocol
    fn get_state_id(&self) -> u16;

    /// Returns a block with properties based on the local state id.
    /// Probably you want to use `get_block` function instead.
    /// This function retrieves a block using a local state ID (from_local_state). In most cases, you want to use the global state to obtain the block instead.
    fn from_local_state(state: u16) -> Option<Self>
    where
        Self: Sized;

    fn is_same_type_as(&self, other: &dyn Block) -> bool {
        self.type_id() == (*other).type_id()
    }

    fn is_same_type_as_self(other: &dyn Block) -> bool
    where
        Self: Sized,
    {
        (*other).type_id() == TypeId::of::<Self>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, FromRepr)]
pub enum BooleanProperty {
    True,
    False,
}

impl From<bool> for BooleanProperty {
    fn from(value: bool) -> Self {
        if value {
            BooleanProperty::True
        } else {
            BooleanProperty::False
        }
    }
}
