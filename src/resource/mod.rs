mod block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockFlags {
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
	pub flags: BitFlags<BlockFlags>,
	pub color: BlockColor,
}

pub fn get_block_types() -> HashMap<String, BlockType> {
	block_types::BLOCK_TYPES
		.iter()
		.map(|(k, v)| (String::from(*k), *v))
		.collect()
}
