use std::fmt::Debug;
use strum::FromRepr;

include!(concat!(env!("OUT_DIR"), "/blocks.rs"));

pub trait Block: Debug {
    /// Gets current Block state based on set fields. Used mostly in protocol
    fn get_state(&self) -> u32;

    /// Returns a block with correct properties by local state id.
    /// Probably you should use `get_block` function instead.
    fn from_local_state(state: u32) -> Option<Self>
    where
        Self: Sized;
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
