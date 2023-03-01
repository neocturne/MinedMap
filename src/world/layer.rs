use anyhow::{Context, Result};
use itertools::iproduct;
use serde::{Deserialize, Serialize};

use super::chunk::Chunk;
use crate::{
	resource::{BlockFlag, BlockType, BlockTypes},
	types::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeight(i32);

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
	block_type: BlockType,
	y: BlockHeight,
	depth: Option<BlockHeight>,
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

/// Fills in a [BlockInfoArray] with the information of the chunk's top
/// block layer
///
/// For each (X, Z) coordinate pair, the topmost opaque block is
/// determined as the block that should be visible on the rendered
/// map. For water blocks, the height of the first non-water block
/// is additionally filled in as the water depth.
pub fn top_layer(chunk: &Chunk, block_types: &BlockTypes) -> Result<Box<BlockInfoArray>> {
	use BLOCKS_PER_CHUNK as N;

	let mut done = 0;
	let mut ret = Box::<BlockInfoArray>::default();

	for ((section_y, section), y, xz) in iproduct!(
		chunk.sections().rev(),
		BlockY::iter().rev(),
		BlockInfoArray::keys()
	) {
		let entry = &mut ret[xz];
		if entry.done() {
			continue;
		}

		let coords = SectionBlockCoords { xz, y };
		let block_id = section.block_id_at(coords)?;
		let Some(block_type) = block_types.get(block_id) else {
			eprintln!("Unknown block type: {}", block_id);
			continue;
		};
		let height = BlockHeight::new(section_y, y)?;
		if !entry.fill(height, block_type) {
			continue;
		}

		assert!(entry.done());

		done += 1;
		if done == N * N {
			break;
		}
	}

	Ok(ret)
}
