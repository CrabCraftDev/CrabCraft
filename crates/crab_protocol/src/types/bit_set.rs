use derive_more::{From, Into};

use crab_protocol::buf::{ReadByteBuf, WriteByteBuf};
use crab_protocol::error::ProtocolError;
use crab_protocol::protocol_serializer::ProtocolSerializer;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
pub struct BitSet {
    pub bits: Vec<bool>,
}

impl BitSet {
    pub fn new(bits: Vec<bool>) -> Self {
        Self { bits }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.bits.len() {
            self.bits.resize(index + 1, false);
        }
        self.bits[index] = value;
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.bits.len() {
            false
        } else {
            self.bits[index]
        }
    }

    pub fn as_long_array(&self) -> Vec<i64> {
        // https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html#toLongArray--
        let mut result = vec![0; self.bits.len().div_ceil(64)];
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                result[i / 64] |= 1 << (i % 64);
            }
        }
        result
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html#toByteArray--
        let mut result = vec![0; self.bits.len().div_ceil(8)];
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                result[i / 8] |= 1 << (i % 8);
            }
        }
        result
    }

    pub fn from_long_array(longs: &[i64]) -> Self {
        let mut bits = Vec::new();
        for &long in longs {
            for i in 0..64 {
                bits.push(long & (1 << i) != 0);
            }
        }
        Self { bits }
    }

    pub fn from_bytes(longs: &[u8]) -> Self {
        let mut bits = Vec::new();
        for &long in longs {
            for i in 0..8 {
                bits.push(long & (1 << i) != 0);
            }
        }
        Self { bits }
    }
}

impl ProtocolSerializer for BitSet {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        buf.put_some_array(&self.as_long_array())?;
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        Ok(BitSet::from_long_array(&buf.get_some_array()?))
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into)]
#[repr(transparent)]
pub struct FixedBitSet<const N: usize>(pub BitSet);

impl<const N: usize> ProtocolSerializer for FixedBitSet<N> {
    fn serialize(&self, buf: &mut impl WriteByteBuf) -> Result<(), ProtocolError> {
        let bits_len = self.0.bits.len();
        if bits_len != N {
            return Err(ProtocolError::InvalidFixedBitSetLength(N, bits_len));
        }
        buf.put(self.0.as_bytes().as_slice());
        Ok(())
    }

    fn deserialize(buf: &mut impl ReadByteBuf) -> Result<Self, ProtocolError> {
        let bytes = buf.get_raw_bytes(N as i32);
        Ok(BitSet::from_bytes(bytes.as_ref()).into())
    }
}
