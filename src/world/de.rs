//! Data structures used to deserialize Minecraft save data

use serde::Deserialize;

/// Element of the `palette` list of 1.18+ [block states](BlockStatesV1_18)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockStatePaletteEntry {
	pub name: String,
}

/// 1.18+ `block_states` element found in a [section](SectionV1_18)
#[derive(Debug, Deserialize)]
pub struct BlockStatesV1_18 {
	pub palette: Vec<BlockStatePaletteEntry>,
	pub data: Option<fastnbt::LongArray>,
}

/// 1.18+ `biomes` element found in a [section](SectionV1_18)
#[derive(Debug, Deserialize)]
pub struct BiomesV1_18 {
	pub palette: Vec<String>,
	pub data: Option<fastnbt::LongArray>,
}

/// Element of the 1.18+ `sections` list found in a [Chunk]
#[derive(Debug, Deserialize)]
pub struct SectionV1_18 {
	#[serde(rename = "Y")]
	pub y: i32,
	pub block_states: BlockStatesV1_18,
	pub biomes: BiomesV1_18,
	#[serde(rename = "BlockLight")]
	pub block_light: Option<fastnbt::ByteArray>,
}

/// Version-specific part of a pre-1.18 [Section](SectionV0)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionV0Variants {
	#[serde(rename_all = "PascalCase")]
	V1_13 {
		block_states: fastnbt::LongArray,
		palette: Vec<BlockStatePaletteEntry>,
	},
	#[serde(rename_all = "PascalCase")]
	V0 {
		blocks: fastnbt::ByteArray,
		data: fastnbt::ByteArray,
	},
	Empty {},
}

/// Pre-1.18 section element found in the [Level](LevelV0) compound
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionV0 {
	pub y: i8,
	pub block_light: Option<fastnbt::ByteArray>,
	#[serde(flatten)]
	pub section: SectionV0Variants,
}

/// Pre-1.18 biome fields found in the [Level](LevelV0) compound
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BiomesV0 {
	IntArray(fastnbt::IntArray),
	ByteArray(fastnbt::ByteArray),
}

/// `Level` compound element found in pre-1.18 [chunks](Chunk)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelV0 {
	#[serde(default)]
	pub sections: Vec<SectionV0>,
	pub biomes: Option<BiomesV0>,
}

/// Version-specific part of a [Chunk] compound
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChunkVariants {
	V1_18 {
		sections: Vec<SectionV1_18>,
	},
	#[serde(rename_all = "PascalCase")]
	V0 {
		level: LevelV0,
	},
}

/// Toplevel compound element of a Minecraft chunk
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chunk {
	pub data_version: Option<u32>,
	#[serde(flatten)]
	pub chunk: ChunkVariants,
}
