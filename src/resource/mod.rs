mod block_types;
mod legacy_block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};

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
pub struct BlockColor(pub [u8; 3]);

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

#[derive(Debug)]
pub struct BlockTypes {
	block_type_map: HashMap<String, BlockType>,
	legacy_block_types: Box<[[BlockType; 16]; 256]>,
}

impl Default for BlockTypes {
	fn default() -> Self {
		let block_type_map: HashMap<_, _> = block_types::BLOCK_TYPES
			.iter()
			.map(|(k, v)| (String::from(*k), *v))
			.collect();
		let legacy_block_types = Box::new(legacy_block_types::LEGACY_BLOCK_TYPES.map(|inner| {
			inner.map(|id| *block_type_map.get(id).expect("Unknown legacy block type"))
		}));

		BlockTypes {
			block_type_map,
			legacy_block_types,
		}
	}
}

impl BlockTypes {
	#[inline]
	pub fn get(&self, id: &str) -> Option<BlockType> {
		let suffix = id.strip_prefix("minecraft:")?;
		self.block_type_map.get(suffix).copied()
	}

	#[inline]
	pub fn get_legacy(&self, id: u8, data: u8) -> Option<BlockType> {
		Some(self.legacy_block_types[id as usize][data as usize])
	}
}
