//! Data structures used to deserialize Minecraft save data

use serde::Deserialize;

/// Element of the `palette` list of 1.18+ [block states](BlockStatesV1_18)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockStatePaletteEntry {
	/// Block type ID
	pub name: String,
}

/// 1.18+ `block_states` element found in a [section](SectionV1_18)
#[derive(Debug, Deserialize)]
pub struct BlockStatesV1_18 {
	/// Palette of block types, indexed by block data
	pub palette: Vec<BlockStatePaletteEntry>,
	/// Block data
	pub data: Option<fastnbt::LongArray>,
}

/// 1.18+ `biomes` element found in a [section](SectionV1_18)
#[derive(Debug, Deserialize)]
pub struct BiomesV1_18 {
	/// Palette of biome types, indexed by biome data
	pub palette: Vec<String>,
	/// Biome data
	pub data: Option<fastnbt::LongArray>,
}

/// Variable part of a [SectionV1_18]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionV1_18Variants {
	/// Populated 1.18+ section
	V1_18 {
		/// Block type data
		block_states: BlockStatesV1_18,
		/// Biome data
		biomes: BiomesV1_18,
		/// Block light data
		#[serde(rename = "BlockLight")]
		block_light: Option<fastnbt::ByteArray>,
	},
	/// Empty section
	Empty {},
}

/// Element of the 1.18+ `sections` list found in a [Chunk]
#[derive(Debug, Deserialize)]
pub struct SectionV1_18 {
	/// Y coordinate
	#[serde(rename = "Y")]
	pub y: i32,
	/// Variable part of section
	#[serde(flatten)]
	pub section: SectionV1_18Variants,
}

/// Version-specific part of a pre-1.18 [Section](SectionV0)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionV0Variants {
	/// v1.13+ data
	#[serde(rename_all = "PascalCase")]
	V1_13 {
		/// Block data
		block_states: fastnbt::LongArray,
		/// Block type palette, indexed by block data
		palette: Vec<BlockStatePaletteEntry>,
	},
	/// Pre-1.13 data
	#[serde(rename_all = "PascalCase")]
	V0 {
		/// Block type data
		blocks: fastnbt::ByteArray,
		/// Block damage / subtype data
		data: fastnbt::ByteArray,
	},
	/// Empty section
	Empty {},
}

/// Pre-1.18 section element found in the [Level](LevelV0) compound
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionV0 {
	/// Y coordinate
	pub y: i8,
	/// Block light data
	pub block_light: Option<fastnbt::ByteArray>,
	/// Version-specific data
	#[serde(flatten)]
	pub section: SectionV0Variants,
}

/// Pre-1.18 biome fields found in the [Level](LevelV0) compound
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BiomesV0 {
	/// Data for Minecraft versions storing biome data as an IntArray
	IntArray(fastnbt::IntArray),
	/// Data for Minecraft versions storing biome data as an ByteArray
	ByteArray(fastnbt::ByteArray),
}

/// `Level` compound element found in pre-1.18 [chunks](Chunk)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelV0 {
	/// Section data
	#[serde(default)]
	pub sections: Vec<SectionV0>,
	/// Biome data
	pub biomes: Option<BiomesV0>,
}

/// Version-specific part of a [Chunk] compound
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChunkVariants {
	/// 1.18+ chunk data
	V1_18 {
		/// List of chunk sections
		sections: Vec<SectionV1_18>,
	},
	/// Pre-1.18 chunk data
	#[serde(rename_all = "PascalCase")]
	V0 {
		/// `Level` field of the chunk
		level: LevelV0,
	},
}

/// Toplevel compound element of a Minecraft chunk
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chunk {
	/// The data version of the chunk
	pub data_version: Option<u32>,
	/// Version-specific chunk data
	#[serde(flatten)]
	pub chunk: ChunkVariants,
}

/// `Data` compound element of level.dat
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDatData {
	/// X coordinate of spawn point for new players
	pub spawn_x: i32,
	/// Z coordinate of spawn point for new players
	pub spawn_z: i32,
}

/// Toplevel compound element of level.dat
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDat {
	/// The `Data` field
	pub data: LevelDatData,
}
