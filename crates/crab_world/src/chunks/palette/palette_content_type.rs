use crate::blocks::block_information::BlockInformation;
use crate::chunks::palette::paletted_storage::calculate_bits_per_entry;
use crate::chunks::section::AMOUNT_OF_BIOMES_IN_CHUNK_SECTION;
use crate::chunks::section::AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION;
use crab_registry::REGISTRY;
use crab_registry::vanilla::Biome;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum PaletteContentType {
    Biomes,
    Blocks,
}

impl PaletteContentType {
    /// Returns the minimum bits per entry required to use `Palette::Indirect`.
    /// This value is hardcoded in the Notchian server.
    pub fn indirect_min_bits(&self) -> u8 {
        match self {
            PaletteContentType::Biomes => 1,
            PaletteContentType::Blocks => 4,
        }
    }

    /// Returns the maximum bits per entry allowed for using `Palette::Indirect`.
    /// Beyond this value, `Palette::Direct` should be used.
    /// This value is hardcoded in the Notchian server.
    pub fn indirect_max_bits(&self) -> u8 {
        match self {
            PaletteContentType::Biomes => 3,
            PaletteContentType::Blocks => 8,
        }
    }

    /// Returns the number of bits necessary to fit all possible values for given type of content
    pub fn direct(&self) -> u8 {
        let value = match self {
            PaletteContentType::Biomes => {
                let registry = REGISTRY.load();
                let registry = registry.get_registry2::<Biome>();
                registry.len()
            }
            PaletteContentType::Blocks => {
                let registry = REGISTRY.load();
                let registry = registry.get_registry2::<BlockInformation>();

                registry.len()
            }
        };
        calculate_bits_per_entry(value)
    }

    pub fn amount_of_entries(&self) -> usize {
        match self {
            PaletteContentType::Biomes => AMOUNT_OF_BIOMES_IN_CHUNK_SECTION as usize,
            PaletteContentType::Blocks => AMOUNT_OF_BLOCKS_IN_CHUNK_SECTION,
        }
    }
}
