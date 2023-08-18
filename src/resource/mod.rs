//! Data describing Minecraft biomes and block types

mod biomes;
mod block_color;
mod legacy_block_types;

#[allow(clippy::missing_docs_in_private_items)] // Generated module
mod block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

/// Flags describing special properties of [BlockType]s
#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BlockFlag {
	/// The block type is opaque
	Opaque,
	/// The block type is colored using biome grass colors
	Grass,
	/// The block type is colored using biome foliage colors
	Foliage,
	/// The block type is birch foliage
	Birch,
	/// The block type is spurce foliage
	Spruce,
	/// The block type is colored using biome water colors
	Water,
}

/// An RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color(pub [u8; 3]);

/// A block type specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockType {
	/// Bit set of [BlockFlag]s describing special properties of the block type
	pub flags: BitFlags<BlockFlag>,
	/// Base color of the block type
	pub color: Color,
}

impl BlockType {
	/// Checks whether a block type has a given [BlockFlag] set
	pub fn is(&self, flag: BlockFlag) -> bool {
		self.flags.contains(flag)
	}
}

/// Used to look up standard Minecraft block types
#[derive(Debug)]
pub struct BlockTypes {
	/// Map of string IDs to block types
	block_type_map: HashMap<String, BlockType>,
	/// Array used to look up old numeric block type and subtype values
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
	/// Resolves a Minecraft 1.13+ string block type ID
	#[inline]
	pub fn get(&self, id: &str) -> Option<BlockType> {
		let suffix = id.strip_prefix("minecraft:")?;
		self.block_type_map.get(suffix).copied()
	}

	/// Resolves a Minecraft pre-1.13 numeric block type ID
	#[inline]
	pub fn get_legacy(&self, id: u8, data: u8) -> Option<BlockType> {
		Some(self.legacy_block_types[id as usize][data as usize])
	}
}

pub use biomes::{Biome, BiomeGrassColorModifier};
pub use block_color::{block_color, needs_biome};

/// Used to look up standard Minecraft biome types
#[derive(Debug)]
pub struct BiomeTypes {
	/// Map of string IDs to biome types
	biome_map: HashMap<String, &'static Biome>,
	/// Array used to look up old numeric biome IDs
	legacy_biomes: Box<[&'static Biome; 256]>,
}

impl Default for BiomeTypes {
	fn default() -> Self {
		let mut biome_map: HashMap<_, _> = biomes::BIOMES
			.iter()
			.map(|(k, v)| (String::from(*k), v))
			.collect();

		for &(old, new) in biomes::BIOME_ALIASES.iter().rev() {
			let biome = biome_map
				.get(new)
				.copied()
				.expect("Biome alias for unknown biome");
			assert!(biome_map.insert(String::from(old), biome).is_none());
		}

		let legacy_biomes = (0..=255)
			.map(|index| {
				let id = biomes::legacy_biome(index);
				*biome_map.get(id).expect("Unknown legacy biome")
			})
			.collect::<Box<[_]>>()
			.try_into()
			.unwrap();

		Self {
			biome_map,
			legacy_biomes,
		}
	}
}

impl BiomeTypes {
	/// Resolves a Minecraft 1.18+ string biome type ID
	#[inline]
	pub fn get(&self, id: &str) -> Option<&Biome> {
		let suffix = id.strip_prefix("minecraft:")?;
		self.biome_map.get(suffix).copied()
	}

	/// Resolves a Minecraft pre-1.18 numeric biome type ID
	#[inline]
	pub fn get_legacy(&self, id: u8) -> Option<&Biome> {
		Some(self.legacy_biomes[id as usize])
	}
}
