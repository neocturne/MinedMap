//! Functions to search the "top" layer of a chunk

use std::num::NonZeroU16;

use anyhow::{Context, Result};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use super::chunk::{Chunk, SectionIterItem};
use crate::{
	resource::{Biome, BlockColor, BlockFlag},
	types::*,
};

/// Height (Y coordinate) of a block
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeight(pub i32);

impl BlockHeight {
	/// Constructs a new [BlockHeight] from section and block Y indices
	///
	/// Returns an error if the resulting coordindate does not fit into
	/// an [i32].
	pub fn new(section: SectionY, block: BlockY) -> Result<Self> {
		let height = section
			.0
			.checked_mul(BLOCKS_PER_CHUNK as i32)
			.and_then(|y| y.checked_add_unsigned(block.0.into()))
			.context("Block height out of bounds")?;
		Ok(BlockHeight(height))
	}
}

/// Array optionally storing a [BlockColor] for each coordinate of a chunk
pub type BlockArray = LayerBlockArray<Option<BlockColor>>;

/// Array optionally storing a biome index for each coordinate of a chunk
///
/// The entries refer to a biome list generated with the top layer data.
/// Indices are stored incremented by 1 to allow using a [NonZeroU16].
pub type BiomeArray = LayerBlockArray<Option<NonZeroU16>>;

/// Array storing a block light value for each coordinate for a chunk
pub type BlockLightArray = LayerBlockArray<u8>;

/// Array optionally storing a depth value for each coordinate for a chunk
pub type DepthArray = LayerBlockArray<Option<BlockHeight>>;

/// References to LayerData entries for a single coordinate pair
struct LayerEntry<'a> {
	/// The block type of the referenced entry
	block: &'a mut Option<BlockColor>,
	/// The biome type of the referenced entry
	biome: &'a mut Option<NonZeroU16>,
	/// The block light of the referenced entry
	block_light: &'a mut u8,
	/// The depth value of the referenced entry
	depth: &'a mut Option<BlockHeight>,
}

impl<'a> LayerEntry<'a> {
	/// Returns true if the entry has not been filled yet (no opaque block has been encountered)
	///
	/// The depth value is filled separately when a non-water block is encountered after the block type
	/// has already been filled.
	fn is_empty(&self) -> bool {
		self.block.is_none()
	}

	/// Returns true if the entry has been filled including its depth (an opaque non-water block has been
	/// encountered)
	fn done(&self) -> bool {
		self.depth.is_some()
	}

	/// Fills in the LayerEntry
	///
	/// Checks whether the passed coordinates point at an opaque or non-water block and
	/// fills in the entry accordingly. Returns true when the block has been filled including its depth.
	fn fill(
		&mut self,
		biome_list: &mut IndexSet<Biome>,
		section: SectionIterItem,
		coords: SectionBlockCoords,
	) -> Result<bool> {
		let Some(block_type) = section
			.section
			.block_at(coords)?
			.filter(|block_type| block_type.block_color.is(BlockFlag::Opaque))
		else {
			if self.is_empty() {
				*self.block_light = section.block_light.block_light_at(coords);
			}

			return Ok(false);
		};

		if self.is_empty() {
			*self.block = Some(block_type.block_color);
			if let Some(biome) = section.biomes.biome_at(section.y, coords)? {
				let (biome_index, _) = biome_list.insert_full(*biome);
				*self.biome = NonZeroU16::new(
					(biome_index + 1)
						.try_into()
						.expect("biome index not in range"),
				);
			}
		}

		if block_type.block_color.is(BlockFlag::Water) {
			return Ok(false);
		}

		let height = BlockHeight::new(section.y, coords.y)?;
		*self.depth = Some(height);

		Ok(true)
	}
}

/// Top layer data
///
/// A LayerData stores block type, biome, block light and depth data for
/// each coordinate of a chunk.
#[derive(Debug, Default)]
pub struct LayerData {
	/// Block type data
	pub blocks: Box<BlockArray>,
	/// Biome data
	pub biomes: Box<BiomeArray>,
	/// Block light data
	pub block_light: Box<BlockLightArray>,
	/// Depth data
	pub depths: Box<DepthArray>,
}

impl LayerData {
	/// Builds a [LayerEntry] referencing the LayerData at a given coordinate pair
	fn entry(&mut self, coords: LayerBlockCoords) -> LayerEntry {
		LayerEntry {
			block: &mut self.blocks[coords],
			biome: &mut self.biomes[coords],
			block_light: &mut self.block_light[coords],
			depth: &mut self.depths[coords],
		}
	}
}

/// Fills in a [LayerData] with the information of the chunk's top
/// block layer
///
/// For each (X, Z) coordinate pair, the topmost opaque block is
/// determined as the block that should be visible on the rendered
/// map. For water blocks, the height of the first non-water block
/// is additionally filled in as the water depth (the block height is
/// used as depth otherwise).
pub fn top_layer(biome_list: &mut IndexSet<Biome>, chunk: &Chunk) -> Result<Option<LayerData>> {
	use BLOCKS_PER_CHUNK as N;

	if chunk.is_empty() {
		return Ok(None);
	}

	let mut done = 0;
	let mut ret = LayerData::default();

	for section in chunk.sections().rev() {
		for y in BlockY::iter().rev() {
			for z in BlockZ::iter() {
				for x in BlockX::iter() {
					let xz = LayerBlockCoords { x, z };

					let mut entry = ret.entry(xz);
					if entry.done() {
						continue;
					}

					let coords = SectionBlockCoords { xz, y };
					if !entry.fill(biome_list, section, coords)? {
						continue;
					}

					assert!(entry.done());
					done += 1;
					if done == N * N {
						break;
					}
				}
			}
		}
	}

	Ok(Some(ret))
}
