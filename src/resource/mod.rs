mod block_types;
mod legacy_block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};

pub use legacy_block_types::LEGACY_BLOCK_TYPES;

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockFlag {
	Opaque,
	Grass,
	Foliage,
	Birch,
	Spruce,
	Water,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockColor(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy)]
pub struct BlockType {
	pub flags: BitFlags<BlockFlag>,
	pub color: BlockColor,
}

pub type BlockTypeMap = HashMap<String, BlockType>;

pub fn block_types() -> BlockTypeMap {
	block_types::BLOCK_TYPES
		.iter()
		.map(|(k, v)| (String::from(*k), *v))
		.collect()
}
