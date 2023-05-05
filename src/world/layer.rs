use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::chunk::{Chunk, SectionIterItem};
use crate::{
	resource::{Biome, BlockFlag, BlockType},
	types::*,
};

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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct BlockInfo {
	pub block_type: Option<BlockType>,
	pub depth: Option<BlockHeight>,
}

pub type BlockInfoArray = LayerBlockArray<BlockInfo>;
pub type BiomeArray = LayerBlockArray<Option<Biome>>;
pub type BlockLightArray = LayerBlockArray<u8>;

struct LayerEntry<'a> {
	block: &'a mut Option<BlockType>,
	biome: &'a mut Option<Biome>,
	block_light: &'a mut u8,
	depth: &'a mut Option<BlockHeight>,
}

impl<'a> LayerEntry<'a> {
	fn is_empty(&self) -> bool {
		self.block.is_none()
	}

	fn done(&self) -> bool {
		self.depth.is_some()
	}

	fn fill(&mut self, y: BlockHeight, block_type: BlockType) -> bool {
		if !block_type.is(BlockFlag::Opaque) {
			return false;
		}

		if self.block.is_none() {
			*self.block = Some(block_type);
		}

		if block_type.is(BlockFlag::Water) {
			return false;
		}

		*self.depth = Some(y);

		true
	}
}

#[derive(Debug, Default)]
pub struct LayerData {
	pub blocks: Box<BlockInfoArray>,
	pub biomes: Box<BiomeArray>,
	pub block_light: Box<BlockLightArray>,
}

impl LayerData {
	fn entry(&mut self, coords: LayerBlockCoords) -> LayerEntry {
		let block_info = &mut self.blocks[coords];
		LayerEntry {
			block: &mut block_info.block_type,
			biome: &mut self.biomes[coords],
			block_light: &mut self.block_light[coords],
			depth: &mut block_info.depth,
		}
	}
}

/// Fills in a [BlockInfoArray] with the information of the chunk's top
/// block layer
///
/// For each (X, Z) coordinate pair, the topmost opaque block is
/// determined as the block that should be visible on the rendered
/// map. For water blocks, the height of the first non-water block
/// is additionally filled in as the water depth.
pub fn top_layer(chunk: &Chunk) -> Result<Option<LayerData>> {
	use BLOCKS_PER_CHUNK as N;

	if chunk.is_empty() {
		return Ok(None);
	}

	let mut done = 0;
	let mut ret = LayerData::default();

	for SectionIterItem {
		y: section_y,
		section,
		biomes,
		block_light,
	} in chunk.sections().rev()
	{
		for y in BlockY::iter().rev() {
			for xz in BlockInfoArray::keys() {
				let mut entry = ret.entry(xz);
				if entry.done() {
					continue;
				}

				let coords = SectionBlockCoords { xz, y };

				'check_block: {
					let Some(block_type) = section.block_at(coords)? else {
						break 'check_block;
					};

					let height = BlockHeight::new(section_y, y)?;
					if !entry.fill(height, block_type) {
						break 'check_block;
					}

					assert!(entry.done());
					done += 1;
				};

				if !entry.is_empty() && entry.biome.is_none() {
					*entry.biome = biomes.biome_at(section_y, coords)?.copied();
				}

				if entry.is_empty() {
					*entry.block_light = block_light.block_light_at(coords);
				}

				if done == N * N {
					break;
				}
			}
		}
	}

	Ok(Some(ret))
}
