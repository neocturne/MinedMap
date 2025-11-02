//! Data structures used to deserialize Minecraft save data

use serde::Deserialize;

use super::text_value::TextValue;

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
	#[serde(default)]
	pub data: Option<fastnbt::LongArray>,
}

/// Variable part of a [SectionV1_18]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionV1_18Variant {
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
	pub section: SectionV1_18Variant,
}

/// Version-specific part of a pre-1.18 [Section](SectionV0)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionV0Variant {
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
	pub section: SectionV0Variant,
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

/// Front/back text of a Minecraft 1.20+ sign block entry
#[derive(Debug, Deserialize)]
pub struct BlockEntitySignV1_20Text {
	/// Lines of sign text
	pub messages: Vec<TextValue>,
	/// Default text color
	pub color: Option<String>,
}

/// A sign (standing or hanging) block entity
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BlockEntitySign {
	/// Pre-1.20 sign block entity
	///
	/// Pre-1.20 signs only have front text.
	#[serde(rename_all = "PascalCase")]
	V0 {
		/// Line 1 of the sign text
		text1: TextValue,
		/// Line 2 of the sign text
		text2: TextValue,
		/// Line 3 of the sign text
		text3: TextValue,
		/// Line 4 of the sign text
		text4: TextValue,
		/// Default text color
		color: Option<String>,
	},
	/// 1.20+ sign block entity
	V1_20 {
		/// The sign's front text
		front_text: BlockEntitySignV1_20Text,
		/// The sign's back text
		back_text: BlockEntitySignV1_20Text,
	},
}

/// Data for different kinds of block entities
#[derive(Debug, Deserialize)]
#[serde(tag = "id")]
pub enum BlockEntityData {
	/// Regular sign
	///
	/// This includes standing signs and signs attached to the side of blocks
	#[serde(rename = "minecraft:sign", alias = "minecraft:standing_sign")]
	Sign(BlockEntitySign),
	/// Hanging sign
	#[serde(rename = "minecraft:hanging_sign")]
	HangingSign(BlockEntitySign),
	/// Other block entity types not handled by MinedMap
	#[serde(other)]
	Other,
}

/// A block entity
///
/// Block entities were called tile entities pre-1.18
#[derive(Debug, Deserialize)]
pub struct BlockEntity {
	/// Entity global X coordinate
	pub x: i32,
	/// Entity global Y coordinate
	pub y: i32,
	/// Entity global Z coordinate
	pub z: i32,
	/// Kind-specific entity data
	#[serde(flatten)]
	pub data: BlockEntityData,
}

/// `Level` compound element found in pre-1.18 [chunks](Chunk)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelV0 {
	/// Section data
	#[serde(default)]
	pub sections: Vec<SectionV0>,
	/// Biome data
	#[serde(default)]
	pub biomes: Option<BiomesV0>,
	/// List of block entities
	#[serde(default)]
	pub tile_entities: Vec<BlockEntity>,
}

/// Version-specific part of a [Chunk] compound
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChunkVariant {
	/// 1.18+ chunk data
	V1_18 {
		/// List of chunk sections
		sections: Vec<SectionV1_18>,
		/// List of block entities
		#[serde(default)]
		block_entities: Vec<BlockEntity>,
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
	pub chunk: ChunkVariant,
}

/// 1.21.9+ `spawn` compound element of level.dat
#[derive(Debug, Deserialize)]
pub struct LevelDatSpawnV1_21_9 {
	/// X/Y/Z coordinate of spawn point for new players
	pub pos: fastnbt::IntArray,
	/// Dimension of the spawn point for new players (for example "minecraft:overworld")
	pub dimension: String,
}

/// `Data` compound element of level.dat
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LevelDatData {
	/// 1.21.9+ `Data` element
	V1_21_9 {
		/// Spawn point for new players
		spawn: LevelDatSpawnV1_21_9,
	},
	/// Pre-1.21.9 `Data` element
	#[serde(rename_all = "PascalCase")]
	V0 {
		/// X coordinate of spawn point for new players
		spawn_x: i32,
		/// Z coordinate of spawn point for new players
		spawn_z: i32,
	},
}

/// Toplevel compound element of level.dat
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelDat {
	/// The `Data` field
	pub data: LevelDatData,
}
