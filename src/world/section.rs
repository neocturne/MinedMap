use anyhow::{bail, Context, Result};

use super::de;

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
}

#[derive(Debug)]
pub struct OldSection<'a> {
	blocks: &'a fastnbt::ByteArray,
	data: &'a fastnbt::ByteArray,
}

impl<'a> OldSection<'a> {
	pub fn new(blocks: &'a fastnbt::ByteArray, data: &'a fastnbt::ByteArray) -> Result<Self> {
		// TODO: Check lengths
		Ok(Self { blocks, data })
	}
}
