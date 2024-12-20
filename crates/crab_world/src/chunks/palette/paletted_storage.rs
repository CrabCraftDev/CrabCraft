use crate::chunks::bit_storage::BitStorage;
use crate::chunks::palette::palette_content_type::PaletteContentType;
use crate::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use crate::error::WorldError;
use bytes::{BufMut, BytesMut};
use crab_protocol::buf::WriteByteBuf;
use nohash_hasher::IntMap;

/// [Palette](https://en.wikipedia.org/wiki/Palette_(computing)) is a structure optimized for storing data in a limited amount of space.
///
/// In Minecraft, palettes are used to store block and biome data within chunks.
///
/// Minecraft utilizes three [types of palettes](https://web.archive.org/web/20241129083318/https://wiki.vg/Chunk_Format#Palette_formats):
/// - **Direct**: Stores IDs for all elements directly.
/// - **Indirect**: Maps IDs to unique data in the palette for efficient storage.
/// - **Single valued**: Stores a single value for all elements in the palette.
#[derive(Debug, Clone, PartialEq)]
pub struct PalettedStorage {
    pub bits_per_entry: u8,
    pub kind: PaletteContentType,
    pub palette: Palette,
}

impl PalettedStorage {
    pub fn new(kind: PaletteContentType, palette: Palette) -> Self {
        let bits_per_entry = match &palette {
            Palette::Single(_) => 0,
            Palette::Indirect {
                values_lookup_table: palette,
                ..
            } => calculate_bits_per_entry(palette.len()).max(kind.indirect_min_bits()),
            Palette::Direct(_) => kind.direct(),
        };

        PalettedStorage {
            bits_per_entry,
            kind,
            palette,
        }
    }

    pub fn single(kind: PaletteContentType, id: u32) -> Self {
        PalettedStorage {
            bits_per_entry: 0,
            kind,
            palette: Palette::Single(id),
        }
    }

    pub fn serialize(&self, bytes: &mut BytesMut) {
        bytes.put_u8(self.bits_per_entry);

        match &self.palette {
            Palette::Single(value) => {
                bytes.put_var_int(*value as i32);
                // Bit storage with len 0
                bytes.put_var_int(0);
            }
            Palette::Indirect {
                values_lookup_table: palette,
                storage,
            } => {
                bytes.put_var_int(palette.len() as i32);
                for element in palette {
                    bytes.put_var_int(*element as i32);
                }
                storage.write(bytes);
            }
            Palette::Direct(storage) => {
                storage.write(bytes);
            }
        }
    }

    pub fn get_at_index(&self, index: usize) -> Option<u32> {
        match &self.palette {
            Palette::Single(value) => Some(*value),
            Palette::Indirect {
                values_lookup_table: palette,
                storage,
            } => {
                let paletted_value = storage.get(index)? as usize;
                Some(palette[paletted_value])
            }
            Palette::Direct(storage) => Some(storage.get(index)? as u32),
        }
    }

    /// Sets value in palette and upscales palette if it is necessary
    ///
    /// If you want to clean up palette, use `shrink_to_fit`
    pub fn set_at_index(&mut self, index: usize, new_value: u32) -> Result<(), WorldError> {
        let paletted_value = self.add_new_value_to_palette(new_value)?;

        match &mut self.palette {
            Palette::Indirect { storage, .. } | Palette::Direct(storage) => {
                storage.set(index, paletted_value as i64)?;
            }
            Palette::Single(..) => {}
        }

        Ok(())
    }

    /// Shrinks the palette to fit all data in the smallest possible size
    ///
    /// It is a bit slow, so use only if necessary (e.g. writing to files)
    pub fn shrink_to_fit(&mut self) -> Result<(), WorldError> {
        let values: Vec<u32> = match &self.palette {
            Palette::Indirect {
                storage,
                values_lookup_table: palette,
            } => storage.iter().map(|value| palette[value as usize]).collect(),
            Palette::Direct(storage) => storage.iter().map(|value| value as u32).collect(),
            _ => return Ok(()),
        };

        let mut unique_values = IntMap::default();
        let mut palette_values = Vec::new();
        for &value in &values {
            if !unique_values.contains_key(&value) {
                unique_values.insert(value, unique_values.len());
                palette_values.push(value);
            }
        }

        if palette_values.len() == 1 {
            self.palette = Palette::Single(palette_values[0]);
            self.bits_per_entry = 0;
            return Ok(());
        }

        let new_bits_per_entry = calculate_bits_per_entry(unique_values.len()).max(self.kind.indirect_min_bits());

        // It has to be direct palette
        if new_bits_per_entry > self.kind.indirect_max_bits() {
            return Ok(());
        }

        let mut new_storage = BitStorage::empty(self.kind.amount_of_entries(), new_bits_per_entry);
        for (index, value) in values.iter().enumerate() {
            new_storage.set(index, unique_values[value] as i64)?
        }

        self.palette = Palette::new(palette_values, new_storage, &self.kind);
        self.bits_per_entry = new_bits_per_entry;

        Ok(())
    }

    /// Adds a new value to the palette, upscaling the storage if necessary.
    fn add_new_value_to_palette(&mut self, new_value: u32) -> Result<u32, WorldError> {
        let result = match &mut self.palette {
            Palette::Single(value) => {
                if *value == new_value {
                    return Ok(0);
                }

                let storage = BitStorage::empty(AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION, self.kind.indirect_min_bits());

                let palette = vec![*value, new_value];
                let new_palette = Palette::new(palette, storage, &self.kind);

                self.palette = new_palette;
                self.bits_per_entry = self.kind.indirect_min_bits();

                1
            }
            Palette::Indirect {
                values_lookup_table: palette,
                storage,
            } => {
                let paletted_value = if let Some(index) = palette.iter().position(|x| x == &new_value) {
                    index
                } else {
                    palette.push(new_value);
                    palette.len() - 1
                };

                let new_bits_per_entry = calculate_bits_per_entry(palette.len()).max(self.kind.indirect_min_bits());
                if new_bits_per_entry > self.kind.indirect_max_bits() {
                    let mut new_storage = BitStorage::empty(self.kind.amount_of_entries(), self.kind.direct());
                    for (index, value) in storage.iter().enumerate() {
                        new_storage.set(index, palette[value as usize] as i64)?
                    }
                    self.bits_per_entry = self.kind.direct();
                    self.palette = Palette::Direct(new_storage);
                } else if storage.bits_per_entry != new_bits_per_entry {
                    *storage = storage.resized(new_bits_per_entry);
                    self.bits_per_entry = new_bits_per_entry;
                };

                paletted_value as u32
            }
            _ => new_value,
        };

        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Palette {
    Single(u32),
    /// This type contains a mapping between real values and their corresponding indices in the storage.
    /// `values_lookup_table` stores the actual values, while `storage` holds the indices that reference these values.
    Indirect {
        values_lookup_table: Vec<u32>,
        storage: BitStorage,
    },
    Direct(BitStorage),
}

impl Palette {
    pub fn new(values_lookup_table: Vec<u32>, storage: BitStorage, palette_kind: &PaletteContentType) -> Self {
        let bits_per_entry = calculate_bits_per_entry(values_lookup_table.len());
        if bits_per_entry == 0 {
            Self::Single(values_lookup_table[0])
        } else if bits_per_entry < palette_kind.indirect_max_bits() {
            Palette::Indirect {
                values_lookup_table,
                storage,
            }
        } else {
            Palette::Direct(storage)
        }
    }

    /// Returns real value for paletted one
    pub fn value_for(&self, paletted_value: u32) -> Option<u32> {
        match &self {
            Palette::Single(value) => Some(*value),
            Palette::Indirect {
                values_lookup_table: palette,
                ..
            } => palette.get(paletted_value as usize).cloned(),
            Palette::Direct(..) => Some(paletted_value),
        }
    }
}

pub fn calculate_bits_per_entry(amount_of_entries: usize) -> u8 {
    (amount_of_entries as f64).log2().ceil() as u8
}
