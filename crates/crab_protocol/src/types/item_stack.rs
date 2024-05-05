use crab_nbt::Nbt;

use crab_protocol_macros::ProtocolSerializer;

use crate::error::ProtocolError;

#[derive(Clone, PartialEq, Debug, ProtocolSerializer)]
pub struct ItemStack {
    pub id: i32,
    pub amount: u8,
    pub nbt: Nbt,
}

impl ItemStack {
    pub fn new(id: i32, amount: u8, nbt: Nbt) -> Result<Self, ProtocolError> {
        Ok(Self { id, amount, nbt })
    }
}
