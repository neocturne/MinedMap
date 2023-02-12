use anyhow::{bail, Context, Result};

use super::de;
use crate::{resource, types::*};

fn palette_bits(len: usize, min: u8, max: u8) -> Option<u8> {
	let mut bits = min;
	while (1 << bits) < len {
		bits += 1;

		if bits > max {
			return None;
		}
	}

	Some(bits)
}

pub trait Section {
	fn get_block_id(&self, coords: BlockCoords) -> Result<&str>;
}

#[derive(Debug)]
pub struct PaletteSectionBiomes<'a> {
	biomes: Option<&'a fastnbt::LongArray>,
	palette: &'a Vec<String>,
	bits: u8,
}

impl<'a> PaletteSectionBiomes<'a> {
	pub fn new(biomes: Option<&'a fastnbt::LongArray>, palette: &'a Vec<String>) -> Result<Self> {
		let bits = palette_bits(palette.len(), 1, 6).context("Unsupported block palette size")?;

		if let Some(biomes) = biomes {
			let biomes_per_word = 64 / bits as usize;
			let expected_length = (64 + biomes_per_word - 1) / biomes_per_word;
			if biomes.len() != expected_length {
				bail!("Invalid section biome data");
			}
		}

		Ok(PaletteSectionBiomes {
			biomes,
			palette,
			bits,
		})
	}
}

#[derive(Debug)]
pub struct PaletteSection<'a> {
	block_states: Option<&'a fastnbt::LongArray>,
	palette: &'a Vec<de::BlockStatePaletteEntry>,
	bits: u8,
	aligned_blocks: bool,
}

impl<'a> PaletteSection<'a> {
	pub fn new(
		data_version: u32,
		block_states: Option<&'a fastnbt::LongArray>,
		palette: &'a Vec<de::BlockStatePaletteEntry>,
	) -> Result<Self> {
		let aligned_blocks = data_version >= 2529;

		let bits = palette_bits(palette.len(), 4, 12).context("Unsupported block palette size")?;

		if let Some(block_states) = block_states {
			let expected_length = if aligned_blocks {
				let blocks_per_word = 64 / bits as usize;
				(4096 + blocks_per_word - 1) / blocks_per_word
			} else {
				64 * bits as usize
			};
			if block_states.len() != expected_length {
				bail!("Invalid section block data");
			}
		}

		Ok(Self {
			block_states,
			palette,
			bits,
			aligned_blocks,
		})
	}

	fn get_palette_index(&self, coords: BlockCoords) -> usize {
		let Some(block_states) = self.block_states else {
			return 0;
		};

		let bits = self.bits as usize;
		let mask = (1 << bits) - 1;

		let offset = coords.offset();

		let shifted = if self.aligned_blocks {
			let blocks_per_word = 64 / bits;
			let (word, shift) = offset.div_rem(blocks_per_word);
			block_states[word] as u64 >> (shift * bits)
		} else {
			let bit_offset = offset * bits;
			let (word, bit_shift) = bit_offset.div_rem(64);

			if bit_shift + bits <= 64 {
				block_states[word] as u64 >> bit_shift
			} else {
				let tmp = (block_states[word + 1] as u64 as u128) << 64
					| block_states[word] as u64 as u128;
				(tmp >> bit_shift) as u64
			}
		};

		(shifted & mask) as usize
	}
}

impl<'a> Section for PaletteSection<'a> {
	fn get_block_id(&self, coords: BlockCoords) -> Result<&str> {
		let index = self.get_palette_index(coords);
		let entry = self
			.palette
			.get(index)
			.context("Palette index out of bounds")?;
		Ok(&entry.name)
	}
}

#[derive(Debug)]
pub struct OldSection<'a> {
	blocks: &'a fastnbt::ByteArray,
	data: &'a fastnbt::ByteArray,
}

impl<'a> OldSection<'a> {
	pub fn new(blocks: &'a fastnbt::ByteArray, data: &'a fastnbt::ByteArray) -> Result<Self> {
		use BLOCKS_PER_CHUNK as N;

		if blocks.len() != N * N * N {
			bail!("Invalid section block data");
		}
		if data.len() != N * N * N / 2 {
			bail!("Invalid section extra data");
		}

		Ok(OldSection { blocks, data })
	}
}

impl<'a> Section for OldSection<'a> {
	fn get_block_id(&self, coords: BlockCoords) -> Result<&str> {
		let offset = coords.offset();
		let block = self.blocks[offset] as u8;

		let (data_offset, data_nibble) = offset.div_rem(2);
		let data_byte = self.data[data_offset] as u8;
		let data = if data_nibble == 1 {
			data_byte >> 4
		} else {
			data_byte & 0xf
		};

		Ok(resource::LEGACY_BLOCK_TYPES[block as usize][data as usize])
	}
}
