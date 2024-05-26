use crate::utils::errors::Error;
use bytes::{BufMut, BytesMut};
use crab_protocol::buf::WriteByteBuf;
use crab_protocol::protocol_serializer::ProtocolSerializer;
use nohash_hasher::NoHashHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub const BIOMES_CONTAINER: PalettedContainer = PalettedContainer {
    indirect_from: 1,
    indirect_to: 3,
    direct_size: 6,
};

pub const BLOCKS_CONTAINER: PalettedContainer = PalettedContainer {
    indirect_from: 4,
    indirect_to: 8,
    direct_size: 15,
};

/// [Palette](https://en.wikipedia.org/wiki/Palette_(computing)) is a structure optimized for storing data in a limited amount of space.
/// In Minecraft, palettes are used to store block and biome data within chunks.
///
/// Minecraft utilizes three [types of palettes](https://wiki.vg/Chunk_Format#Palette_formats):
/// - **Direct**: Stores IDs for all elements directly.
/// - **Indirect**: Maps IDs to unique values in the palette for efficient storage.
/// - **Single valued**: Stores a single value for all elements in the palette.
#[derive(Clone, Debug)]
pub struct PalettedContainer {
    indirect_from: usize,
    indirect_to: usize,
    direct_size: usize,
}

impl PalettedContainer {
    pub fn serialize_palette(&self, entries: Vec<i32>, bytes: &mut BytesMut) -> Result<(), Error> {
        let mut unique_values: HashMap<i32, usize, BuildHasherDefault<NoHashHasher<u8>>> =
            HashMap::with_hasher(BuildHasherDefault::default());
        let mut insertion_order = Vec::new();

        for &value in &entries {
            // If we have more unique values than we can store in the indirect palette, we can stop
            if unique_values.len() > self.indirect_to * self.indirect_to {
                break;
            }
            if !unique_values.contains_key(&value) {
                insertion_order.push(value);
                unique_values.insert(value, unique_values.len());
            }
        }

        let bits_per_entry = (unique_values.len() as f32).sqrt().ceil() as usize;
        if bits_per_entry == 1 {
            Self::serialize_single_valued_palette(bytes, entries[0]);
        } else if bits_per_entry <= self.indirect_to {
            let entries: Vec<i32> = entries.iter().map(|v| unique_values[v] as i32).collect();
            Self::serialize_indirect_palette(
                bytes,
                &entries,
                &insertion_order,
                bits_per_entry.max(self.indirect_from),
            )?;
        } else {
            Self::write_entries_as_longs(bytes, &entries, self.direct_size)?;
        };
        Ok(())
    }

    fn serialize_indirect_palette(
        bytes: &mut BytesMut,
        entries: &[i32],
        insertion_order: &[i32],
        bits_per_entry: usize,
    ) -> Result<(), Error> {
        bytes.put_u8(bits_per_entry as u8);
        bytes.put_var_int(insertion_order.len() as i32);
        for &value in insertion_order {
            bytes.put_var_int(value);
        }
        Self::write_entries_as_longs(bytes, entries, bits_per_entry)?;
        Ok(())
    }

    fn serialize_single_valued_palette(chunk_section: &mut BytesMut, entry: i32) {
        chunk_section.put_u8(0);
        chunk_section.put_var_int(entry);
        chunk_section.put_var_int(0);
    }

    pub fn write_entries_as_longs(bytes: &mut BytesMut, entries: &[i32], bits_per_entry: usize) -> Result<(), Error> {
        let longs = get_entries_as_longs(entries, bits_per_entry);
        longs.serialize(bytes)?;
        Ok(())
    }
}

pub fn get_entries_as_longs(entries: &[i32], bits_per_entry: usize) -> Vec<i64> {
    let amount_of_numbers_in_long = u64::BITS / bits_per_entry as u32;
    let amount_of_longs = (entries.len() as f32 / amount_of_numbers_in_long as f32).ceil() as i32;

    let mut longs = Vec::with_capacity(amount_of_longs as usize);
    for entries_chunk in entries.chunks(amount_of_numbers_in_long as usize) {
        longs.push(generate_data_array_long(entries_chunk, bits_per_entry) as i64);
    }
    longs
}

pub fn generate_data_array_long(numbers: &[i32], bits_per_number: usize) -> u64 {
    let mut result_number = 0;
    for (i, &number) in numbers.iter().enumerate() {
        result_number |= (number as u64) << (i * bits_per_number);
    }
    result_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Buf;
    use crab_protocol::buf::ReadByteBuf;

    const TEST_ENTRIES: [i32; 4] = [1, 2, 3, 4];
    const TEST_ENTRIES_AS_BYTES: u64 = 0b0000000000000100000000000000001100000000000000100000000000000001;

    #[test]
    fn test_serialize_indirect() {
        let mut bytes = BytesMut::new();
        BIOMES_CONTAINER
            .serialize_palette(TEST_ENTRIES.to_vec(), &mut bytes)
            .unwrap();
        assert_eq!(bytes.get_u8(), 2);
        assert_eq!(bytes.get_var_int().unwrap(), 4);
        for value in 1..5 {
            assert_eq!(bytes.get_var_int().unwrap(), value);
        }
    }

    #[test]
    fn test_get_entries_as_longs() {
        let numbers = [TEST_ENTRIES; 3].concat();
        let result = get_entries_as_longs(&numbers, 16);
        assert_eq!(result, vec![TEST_ENTRIES_AS_BYTES as i64; 3]);
    }

    #[test]
    fn test_generate_data_array_long() {
        let result = generate_data_array_long(&TEST_ENTRIES, 16);
        assert_eq!(result, TEST_ENTRIES_AS_BYTES);
    }
}
