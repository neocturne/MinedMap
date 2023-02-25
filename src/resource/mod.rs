mod block_types;
mod legacy_block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};

pub use legacy_block_types::LEGACY_BLOCK_TYPES;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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

fn serialize_block_flags<S>(flags: &BitFlags<BlockFlag>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	flags.bits().serialize(serializer)
}
fn deserialize_block_flags<'de, D>(deserializer: D) -> Result<BitFlags<BlockFlag>, D::Error>
where
	D: Deserializer<'de>,
{
	let bits = u8::deserialize(deserializer)?;
	BitFlags::<BlockFlag>::from_bits(bits).map_err(de::Error::custom)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockColor(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockType {
	#[serde(
		serialize_with = "serialize_block_flags",
		deserialize_with = "deserialize_block_flags"
	)]
	pub flags: BitFlags<BlockFlag>,
	pub color: BlockColor,
}

impl BlockType {
	pub fn is(&self, flag: BlockFlag) -> bool {
		self.flags.contains(flag)
	}
}

pub type BlockTypeMap = HashMap<String, BlockType>;

pub fn block_types() -> BlockTypeMap {
	block_types::BLOCK_TYPES
		.iter()
		.map(|(k, v)| (String::from(*k), *v))
		.collect()
}
