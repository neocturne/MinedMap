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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockInfo {
	pub block_type: BlockType,
	pub y: BlockHeight,
	pub depth: Option<BlockHeight>,
}

/// Helper methods for [BlockInfo]
trait OptionBlockInfoExt {
	/// Checks if a [BlockInfo] has been filled in completely
	///
	/// Helper used by [top_layer]
	fn done(&self) -> bool;

	/// Fills in a [BlockInfo] based on a [BlockType]
	///
	/// Only fills in data if the block is part of the visible top layer
	/// of the rendered map.
	///
	/// Must be called on an incomplete [BlockInfo] entry. Returns `true`
	/// if the entry has been filled in completely.
	fn fill(&mut self, y: BlockHeight, block_type: BlockType) -> bool;
}

impl OptionBlockInfoExt for Option<BlockInfo> {
	fn done(&self) -> bool {
		let Some(info) = self else {
			return false;
		};

		info.depth.is_some()
	}

	fn fill(&mut self, y: BlockHeight, block_type: BlockType) -> bool {
		if !block_type.is(BlockFlag::Opaque) {
			return false;
		}

		if self.is_none() {
			*self = Some(BlockInfo {
				block_type,
				y,
				depth: None,
			});
		}

		if block_type.is(BlockFlag::Water) {
			return false;
		}

		let info = self.as_mut().unwrap();
		info.depth = Some(y);

		true
	}
}

pub type BlockInfoArray = LayerBlockArray<Option<BlockInfo>>;
pub type BiomeArray = LayerBlockArray<Option<Biome>>;
pub type BlockLightArray = LayerBlockArray<u8>;

#[derive(Debug, Default)]
pub struct LayerData {
	pub blocks: Box<BlockInfoArray>,
	pub biomes: Box<BiomeArray>,
	pub block_light: Box<BlockLightArray>,
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
				let entry = &mut ret.blocks[xz];
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

				let biome_entry = &mut ret.biomes[xz];
				if !entry.is_none() && biome_entry.is_none() {
					*biome_entry = biomes.biome_at(section_y, coords)?.copied();
				}

				if entry.is_none() {
					ret.block_light[xz] = block_light.block_light_at(coords);
				}

				if done == N * N {
					break;
				}
			}
		}
	}

	Ok(Some(ret))
}
