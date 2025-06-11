use crate::error::WorldError;
use bytes::{BufMut, BytesMut};

/// Stores n-bits data inside packed vector of longs
/// Thanks to Feather for [original implementation](https://github.com/feather-rs/feather/blob/main/feather/base/src/chunk/packed_array.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct BitStorage {
    pub amount_of_entries: usize,
    pub bits_per_entry: u8,
    pub data: Vec<i64>,
}

impl BitStorage {
    pub fn new(amount_of_entries: usize, data: Vec<i64>) -> Self {
        let bits_per_entry = data.len() * 64 / amount_of_entries;
        Self {
            amount_of_entries,
            bits_per_entry: bits_per_entry as u8,
            data,
        }
    }

    pub fn empty(amount_of_entries: usize, bits_per_entry: u8) -> Self {
        let mut this = Self {
            amount_of_entries,
            bits_per_entry,
            data: Vec::new(),
        };

        let numbers_in_long = this.amount_of_values_in_long();
        let needed_numbers = amount_of_entries.div_ceil(numbers_in_long);
        this.data = vec![0i64; needed_numbers];
        this
    }

    pub fn write(&self, bytes: &mut BytesMut) {
        for value in &self.data {
            bytes.put_i64(*value);
        }
    }

    pub fn fill(&mut self, value: i64) -> Result<(), WorldError> {
        let mask = self.mask();
        if value > mask {
            return Err(WorldError::ValueExceedsMask(value, mask));
        }

        let mut x = 0;
        for i in 0..self.amount_of_values_in_long() {
            x |= value << (i * self.bits_per_entry as usize);
        }

        self.data.fill(x);

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<i64> {
        if index >= self.amount_of_entries {
            return None;
        }
        let (value_index, bit_index) = self.position(index);
        let i64 = self.data[value_index];
        Some((i64 >> bit_index) & self.mask())
    }

    pub fn set(&mut self, index: usize, value: i64) -> Result<(), WorldError> {
        if index >= self.amount_of_entries {
            return Err(WorldError::IndexOutsideOfBounds(index, self.amount_of_entries));
        }

        let mask = self.mask();
        if value > mask {
            return Err(WorldError::ValueExceedsMask(value, mask));
        }

        let (value_index, bit_index) = self.position(index);

        let i64 = &mut self.data[value_index];
        *i64 &= !(mask << bit_index);
        *i64 |= value << bit_index;

        Ok(())
    }

    /// Resizes this packed array to a new bits per value.
    pub fn resized(&self, new_bits_per_value: u8) -> Self {
        Self::from_iter(self.iter(), new_bits_per_value)
    }

    pub fn count<F>(&self, condition: F) -> usize
    where
        F: Fn(i64) -> bool,
    {
        let mask = self.mask();
        let numbers_in_long = self.amount_of_values_in_long();
        let bits_per_entry = self.bits_per_entry as usize;

        let mut count = 0;

        let length = self.amount_of_entries;

        for (n, &i64_value) in self.data.iter().enumerate() {
            if n >= length {
                break;
            }

            for i in 0..numbers_in_long {
                let extracted = (i64_value >> (i * bits_per_entry)) & mask;
                if condition(extracted) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        let numbers_in_long = self.amount_of_values_in_long();
        let bits_per_entry = self.bits_per_entry as i64;
        let mask = self.mask();
        let length = self.amount_of_entries;

        self.data
            .iter()
            .flat_map(move |&i64| (0..numbers_in_long).map(move |i| (i64 >> (i as i64 * bits_per_entry)) & mask))
            .take(length)
    }

    pub fn from_iter(iter: impl IntoIterator<Item = i64>, bits_per_value: u8) -> Self {
        let iter = iter.into_iter();
        let mut data = Vec::with_capacity(iter.size_hint().0);

        let mut current_u64 = 0i64;
        let mut current_offset = 0;
        let mut amount_of_entries = 0;

        for value in iter {
            current_u64 |= value << current_offset;

            current_offset += bits_per_value;
            if current_offset > 64 - bits_per_value {
                data.push(current_u64);
                current_offset = 0;
                current_u64 = 0;
            }

            amount_of_entries += 1;
        }

        if current_offset != 0 {
            data.push(current_u64);
        }

        Self {
            amount_of_entries,
            bits_per_entry: bits_per_value,
            data,
        }
    }

    /// Creates bit mask for accessing only bits_per_entry bits from number
    fn mask(&self) -> i64 {
        (1 << self.bits_per_entry) - 1
    }

    fn position(&self, index: usize) -> (usize, usize) {
        let value_index = index / self.amount_of_values_in_long();
        let bit_index = (index % self.amount_of_values_in_long()) * self.bits_per_entry as usize;

        (value_index, bit_index)
    }

    fn amount_of_values_in_long(&self) -> usize {
        (i64::BITS / self.bits_per_entry as u32) as usize
    }
}
