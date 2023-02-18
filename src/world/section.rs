use anyhow::{bail, Context, Result};
use num_integer::div_rem;

use super::de;
use crate::{resource, types::*};

/// Determine the number of bits required for indexing into a palette of a given length
///
/// This is basically a base-2 logarithm, with clamping to a minimum value and
/// check against a maximum value. If the result would be greater than the passed
/// `max` value, [None] is returned.
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

/// Trait for common functions of [SectionV1_13] and [SectionV0]
pub trait Section {
	fn block_id_at(&self, coords: SectionBlockCoords) -> Result<&str>;
}

/// Minecraft v1.18+ section biome data
///
/// The biome data is part of the section structure in Minecraft v1.18+, with
/// the biomes laid out as an array of indices into a palette, similar to the
/// v1.13+ block data.
#[derive(Debug)]
pub struct BiomesV18<'a> {
	_biomes: Option<&'a fastnbt::LongArray>,
	_palette: &'a Vec<String>,
	_bits: u8,
}

impl<'a> BiomesV18<'a> {
	/// Constructs a new [BiomesV18] from deserialized data structures
	pub fn new(biomes: Option<&'a fastnbt::LongArray>, palette: &'a Vec<String>) -> Result<Self> {
		let bits = palette_bits(palette.len(), 1, 6).context("Unsupported block palette size")?;

		if let Some(biomes) = biomes {
			let biomes_per_word = 64 / bits as usize;
			let expected_length = (64 + biomes_per_word - 1) / biomes_per_word;
			if biomes.len() != expected_length {
				bail!("Invalid section biome data");
			}
		}

		Ok(BiomesV18 {
			_biomes: biomes,
			_palette: palette,
			_bits: bits,
		})
	}
}

/// Minecraft v1.13+ section block data
#[derive(Debug)]
pub struct SectionV1_13<'a> {
	block_states: Option<&'a fastnbt::LongArray>,
	palette: &'a Vec<de::BlockStatePaletteEntry>,
	bits: u8,
	aligned_blocks: bool,
}

impl<'a> SectionV1_13<'a> {
	/// Constructs a new [SectionV1_13] from deserialized data structures
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

	/// Looks up the block type palette index at the given coordinates
	fn palette_index_at(&self, coords: SectionBlockCoords) -> usize {
		let Some(block_states) = self.block_states else {
			return 0;
		};

		let bits = self.bits as usize;
		let mask = (1 << bits) - 1;

		let offset = coords.offset();

		let shifted = if self.aligned_blocks {
			let blocks_per_word = 64 / bits;
			let (word, shift) = div_rem(offset, blocks_per_word);
			block_states[word] as u64 >> (shift * bits)
		} else {
			let bit_offset = offset * bits;
			let (word, bit_shift) = div_rem(bit_offset, 64);

			let mut tmp = (block_states[word] as u64) >> bit_shift;
			if bit_shift + bits > 64 {
				tmp |= (block_states[word + 1] as u64) << (64 - bit_shift);
			}
			tmp
		};

		(shifted & mask) as usize
	}
}

impl<'a> Section for SectionV1_13<'a> {
	fn block_id_at(&self, coords: SectionBlockCoords) -> Result<&str> {
		let index = self.palette_index_at(coords);
		let entry = self
			.palette
			.get(index)
			.context("Palette index out of bounds")?;
		Ok(&entry.name)
	}
}

/// Pre-1.13 section block data
#[derive(Debug)]
pub struct SectionV0<'a> {
	blocks: &'a fastnbt::ByteArray,
	data: &'a fastnbt::ByteArray,
}

impl<'a> SectionV0<'a> {
	/// Constructs a new [SectionV0] from deserialized data structures
	pub fn new(blocks: &'a fastnbt::ByteArray, data: &'a fastnbt::ByteArray) -> Result<Self> {
		use BLOCKS_PER_CHUNK as N;

		if blocks.len() != N * N * N {
			bail!("Invalid section block data");
		}
		if data.len() != N * N * N / 2 {
			bail!("Invalid section extra data");
		}

		Ok(SectionV0 { blocks, data })
	}
}

impl<'a> Section for SectionV0<'a> {
	fn block_id_at(&self, coords: SectionBlockCoords) -> Result<&str> {
		let offset = coords.offset();
		let block = self.blocks[offset] as u8;

		let (data_offset, data_nibble) = div_rem(offset, 2);
		let data_byte = self.data[data_offset] as u8;
		let data = if data_nibble == 1 {
			data_byte >> 4
		} else {
			data_byte & 0xf
		};

		Ok(resource::LEGACY_BLOCK_TYPES[block as usize][data as usize])
	}
}
