//! Higher-level interfaces to section data
//!
//! The data types in this module attempt to provide interfaces abstracting
//! over different data versions as much as possible.

use std::fmt::Debug;

use anyhow::{Context, Result, bail};
use num_integer::div_rem;
use tracing::debug;

use super::de;
use crate::{
	resource::{Biome, BiomeTypes, BlockType, BlockTypes},
	types::*,
};

use BLOCKS_PER_CHUNK as N;
/// Maximum height of pre-1.18 levels
const HEIGHT: usize = 256;
/// Number of biome entries per chunk in each direction
const BN: usize = N >> 2;
/// Pre-1.18 height of level measured in 4-block spans (resolution of 1.15+ biome data)
const BHEIGHT: usize = HEIGHT >> 2;

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
pub trait Section: Debug {
	/// Returns the [BlockType] at a coordinate tuple inside the section
	fn block_at(&self, coords: SectionBlockCoords) -> Result<Option<&BlockType>>;
}

/// Minecraft v1.13+ section block data
#[derive(Debug)]
pub struct SectionV1_13<'a> {
	/// Packed block type data
	block_states: Option<&'a [i64]>,
	/// List of block types indexed by entries encoded in *block_states*
	palette: Vec<Option<&'a BlockType>>,
	/// Number of bits per block in *block_states*
	bits: u8,
	/// Set to true if packed block entries in *block_states* are aligned to i64
	///
	/// In older data formats, entries are unaligned and a single block can span
	/// two i64 entries.
	aligned_blocks: bool,
}

impl<'a> SectionV1_13<'a> {
	/// Constructs a new [SectionV1_13] from deserialized data structures
	///
	/// The block IDs in the section's palette are resolved to their [BlockType]s
	/// to allow for faster lookup later.
	pub fn new(
		data_version: u32,
		block_states: Option<&'a [i64]>,
		palette: &'a [de::BlockStatePaletteEntry],
		block_types: &'a BlockTypes,
	) -> Result<(Self, bool)> {
		let aligned_blocks = data_version >= 2529;

		let bits = palette_bits(palette.len(), 4, 12).context("Unsupported block palette size")?;

		if let Some(block_states) = block_states {
			let expected_length = if aligned_blocks {
				let blocks_per_word = 64 / bits as usize;
				4096usize.div_ceil(blocks_per_word)
			} else {
				64 * bits as usize
			};
			if block_states.len() != expected_length {
				bail!("Invalid section block data");
			}
		}

		let mut has_unknown = false;

		let palette_types = palette
			.iter()
			.map(|entry| {
				let block_type = block_types.get(&entry.name);
				if block_type.is_none() {
					debug!("Unknown block type: {}", entry.name);
					has_unknown = true;
				}
				block_type
			})
			.collect();

		Ok((
			Self {
				block_states,
				palette: palette_types,
				bits,
				aligned_blocks,
			},
			has_unknown,
		))
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

impl Section for SectionV1_13<'_> {
	fn block_at(&self, coords: SectionBlockCoords) -> Result<Option<&BlockType>> {
		let index = self.palette_index_at(coords);
		Ok(*self
			.palette
			.get(index)
			.context("Palette index out of bounds")?)
	}
}

/// Pre-1.13 section block data
#[derive(Debug)]
pub struct SectionV0<'a> {
	/// Block type data
	///
	/// Each i8 entry corresponds to a block in the 16x16x16 section
	blocks: &'a [i8],
	/// Block damage/subtype data
	///
	/// Uses 4 bits for each block in the 16x16x16 section
	data: &'a [i8],
	/// Used to look up block type IDs
	block_types: &'a BlockTypes,
}

impl<'a> SectionV0<'a> {
	/// Constructs a new [SectionV0] from deserialized data structures
	pub fn new(blocks: &'a [i8], data: &'a [i8], block_types: &'a BlockTypes) -> Result<Self> {
		if blocks.len() != N * N * N {
			bail!("Invalid section block data");
		}
		if data.len() != N * N * N / 2 {
			bail!("Invalid section extra data");
		}

		Ok(SectionV0 {
			blocks,
			data,
			block_types,
		})
	}
}

impl Section for SectionV0<'_> {
	fn block_at(&self, coords: SectionBlockCoords) -> Result<Option<&BlockType>> {
		let offset = coords.offset();
		let block = self.blocks[offset] as u8;

		let (data_offset, data_nibble) = div_rem(offset, 2);
		let data_byte = self.data[data_offset] as u8;
		let data = if data_nibble == 1 {
			data_byte >> 4
		} else {
			data_byte & 0xf
		};

		Ok(self.block_types.get_legacy(block, data))
	}
}

/// Trait for common functions of [BiomesV1_18] and [BiomesV0]
pub trait Biomes: Debug {
	/// Returns the [Biome] at a coordinate tuple inside the chunk
	fn biome_at(&self, section: SectionY, coords: SectionBlockCoords) -> Result<&Biome>;
}

/// Minecraft v1.18+ section biome data
///
/// The biome data is part of the section structure in Minecraft v1.18+, with
/// the biomes laid out as an array of indices into a palette, similar to the
/// v1.13+ block data.
#[derive(Debug)]
pub struct BiomesV1_18<'a> {
	/// Packed biome data
	///
	/// Each entry specifies the biome of a 4x4x4 block area.
	///
	/// Unlike block type data in [SectionV1_13], biome data is always aligned
	/// to whole i64 values.
	biomes: Option<&'a [i64]>,
	/// Biome palette indexed by entries encoded in *biomes*
	palette: Vec<&'a Biome>,
	/// Number of bits used for each entry in *biomes*
	bits: u8,
}

impl<'a> BiomesV1_18<'a> {
	/// Constructs a new [BiomesV1_18] from deserialized data structures
	pub fn new(
		biomes: Option<&'a [i64]>,
		palette: &'a [String],
		biome_types: &'a BiomeTypes,
	) -> Result<(Self, bool)> {
		let bits = palette_bits(palette.len(), 1, 6).context("Unsupported block palette size")?;

		if let Some(biomes) = biomes {
			let biomes_per_word = 64 / bits as usize;
			let expected_length = 64usize.div_ceil(biomes_per_word);
			if biomes.len() != expected_length {
				bail!("Invalid section biome data");
			}
		}

		let mut has_unknown = false;

		let palette_types = palette
			.iter()
			.map(|entry| {
				biome_types.get(entry).unwrap_or_else(|| {
					debug!("Unknown biome type: {}", entry);
					has_unknown = true;
					biome_types.get_fallback()
				})
			})
			.collect();

		Ok((
			BiomesV1_18 {
				biomes,
				palette: palette_types,
				bits,
			},
			has_unknown,
		))
	}

	/// Looks up the block type palette index at the given coordinates
	fn palette_index_at(&self, coords: SectionBlockCoords) -> usize {
		let Some(biomes) = self.biomes else {
			return 0;
		};

		let bits = self.bits as usize;
		let mask = (1 << bits) - 1;

		let x = (coords.xz.x.0 >> 2) as usize;
		let y = (coords.y.0 >> 2) as usize;
		let z = (coords.xz.z.0 >> 2) as usize;
		let offset = BN * BN * y + BN * z + x;

		let blocks_per_word = 64 / bits;
		let (word, shift) = div_rem(offset, blocks_per_word);
		let shifted = biomes[word] as u64 >> (shift * bits);

		(shifted & mask) as usize
	}
}

impl Biomes for BiomesV1_18<'_> {
	fn biome_at(&self, _section: SectionY, coords: SectionBlockCoords) -> Result<&Biome> {
		let index = self.palette_index_at(coords);
		Ok(*self
			.palette
			.get(index)
			.context("Palette index out of bounds")?)
	}
}

/// Pre-v1.18 section biome data variants
///
/// There are a 3 formats for biome data that were used in
/// different pre-v1.18 Minecraft versions
#[derive(Debug)]
enum BiomesV0Data<'a> {
	/// Biome data stored as IntArray in 1.15+ format
	///
	/// Minecraft 1.15 switched to 3-dimensional biome information, but reduced
	/// the resolution to only use one entry for every 4x4x4 block area.
	IntArrayV15(&'a fastnbt::IntArray),
	/// Biome data stored as IntArray in some pre-1.15 versions
	IntArrayV0(&'a fastnbt::IntArray),
	/// Biome data stored as ByteArray in some pre-1.15 versions
	ByteArray(&'a fastnbt::ByteArray),
}

/// Pre-v1.18 section biome data
#[derive(Debug)]
pub struct BiomesV0<'a> {
	/// Biome data from save data
	data: BiomesV0Data<'a>,
	/// Used to look up biome IDs
	biome_types: &'a BiomeTypes,
}

impl<'a> BiomesV0<'a> {
	/// Constructs a new [BiomesV0] from deserialized data structures
	pub fn new(biomes: Option<&'a de::BiomesV0>, biome_types: &'a BiomeTypes) -> Result<Self> {
		let data = match biomes {
			Some(de::BiomesV0::IntArray(data)) if data.len() == BN * BN * BHEIGHT => {
				BiomesV0Data::IntArrayV15(data)
			}
			Some(de::BiomesV0::IntArray(data)) if data.len() == N * N => {
				BiomesV0Data::IntArrayV0(data)
			}
			Some(de::BiomesV0::ByteArray(data)) if data.len() == N * N => {
				BiomesV0Data::ByteArray(data)
			}
			_ => bail!("Invalid biome data"),
		};
		Ok(BiomesV0 { data, biome_types })
	}
}

impl Biomes for BiomesV0<'_> {
	fn biome_at(&self, section: SectionY, coords: SectionBlockCoords) -> Result<&Biome> {
		let id = match self.data {
			BiomesV0Data::IntArrayV15(data) => {
				let LayerBlockCoords { x, z } = coords.xz;
				let y = section
					.0
					.checked_mul(BLOCKS_PER_CHUNK as i32)
					.and_then(|y| y.checked_add_unsigned(coords.y.0.into()))
					.filter(|&height| height >= 0 && (height as usize) < HEIGHT)
					.context("Y coordinate out of range")? as usize;
				let offset = (y >> 2) * BN * BN + (z.0 >> 2) as usize * BN + (x.0 >> 2) as usize;
				let id = data[offset] as u32;
				id.try_into().context("Biome index out of range")?
			}
			BiomesV0Data::IntArrayV0(data) => {
				let id = data[coords.xz.offset()] as u32;
				id.try_into().context("Biome index out of range")?
			}
			BiomesV0Data::ByteArray(data) => data[coords.xz.offset()] as u8,
		};
		Ok(self
			.biome_types
			.get_legacy(id)
			.unwrap_or(self.biome_types.get_fallback()))
	}
}

/// Wrapper around chunk block light data array
#[derive(Debug, Clone, Copy)]
pub struct BlockLight<'a>(Option<&'a [i8]>);

impl<'a> BlockLight<'a> {
	/// Creates a new [BlockLight], checking validity
	pub fn new(block_light: Option<&'a [i8]>) -> Result<Self> {
		if let Some(block_light) = block_light {
			if block_light.len() != N * N * N / 2 {
				bail!("Invalid section block light data");
			}
		}
		Ok(BlockLight(block_light))
	}

	/// Returns the block light value at the given coordinates
	pub fn block_light_at(&self, coords: SectionBlockCoords) -> u8 {
		let Some(block_light) = self.0 else {
			return 0;
		};

		let (offset, nibble) = div_rem(coords.offset(), 2);
		let byte = block_light[offset] as u8;

		if nibble == 1 { byte >> 4 } else { byte & 0xf }
	}
}
