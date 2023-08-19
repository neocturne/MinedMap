mod biomes;
mod block_color;
mod block_types;
mod legacy_block_types;

use std::collections::HashMap;

use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BlockFlag {
	Opaque,
	Grass,
	Foliage,
	Birch,
	Spruce,
	Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color(pub [u8; 3]);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockType {
	pub flags: BitFlags<BlockFlag>,
	pub color: Color,
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

pub use biomes::{Biome, BiomeGrassColorModifier};
pub use block_color::{block_color, needs_biome};

#[derive(Debug)]
pub struct BiomeTypes {
	biome_map: HashMap<String, &'static Biome>,
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
	#[inline]
	pub fn get(&self, id: &str) -> Option<&Biome> {
		let suffix = id.strip_prefix("minecraft:")?;
		self.biome_map.get(suffix).copied()
	}

	#[inline]
	pub fn get_legacy(&self, id: u8) -> Option<&Biome> {
		Some(self.legacy_biomes[id as usize])
	}
}
