#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod biomes;
mod block_color;
mod legacy_biomes;
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
	/// The block type is spruce foliage
	Spruce,
	/// The block type is colored using biome water colors
	Water,
	/// The block type is a wall sign
	///
	/// The WallSign flag is used to distinguish wall signs from
	/// freestanding or -hanging signs.
	WallSign,
}

/// An RGB color with u8 components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color(pub [u8; 3]);

/// An RGB color with f32 components
pub type Colorf = glam::Vec3;

/// A block type specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockColor {
	/// Bit set of [BlockFlag]s describing special properties of the block type
	pub flags: BitFlags<BlockFlag>,
	/// Base color of the block type
	pub color: Color,
}

impl BlockColor {
	/// Checks whether a block color has a given [BlockFlag] set
	#[inline]
	pub fn is(&self, flag: BlockFlag) -> bool {
		self.flags.contains(flag)
	}
}

/// A block type specification (for use in constants)
#[derive(Debug, Clone)]
struct ConstBlockType {
	/// Determines the rendered color of the block type
	pub block_color: BlockColor,
	/// Material of a sign block
	pub sign_material: Option<&'static str>,
}

/// A block type specification
#[derive(Debug, Clone)]
pub struct BlockType {
	/// Determines the rendered color of the block type
	pub block_color: BlockColor,
	/// Material of a sign block
	pub sign_material: Option<String>,
}

impl From<&ConstBlockType> for BlockType {
	fn from(value: &ConstBlockType) -> Self {
		BlockType {
			block_color: value.block_color,
			sign_material: value.sign_material.map(String::from),
		}
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
			.map(|(k, v)| (String::from(*k), BlockType::from(v)))
			.collect();
		let legacy_block_types = Box::new(legacy_block_types::LEGACY_BLOCK_TYPES.map(|inner| {
			inner.map(|id| {
				block_type_map
					.get(id)
					.expect("Unknown legacy block type")
					.clone()
			})
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
	pub fn get(&self, id: &str) -> Option<&BlockType> {
		let suffix = id.strip_prefix("minecraft:")?;
		self.block_type_map.get(suffix)
	}

	/// Resolves a Minecraft pre-1.13 numeric block type ID
	#[inline]
	pub fn get_legacy(&self, id: u8, data: u8) -> Option<&BlockType> {
		Some(&self.legacy_block_types[id as usize][data as usize])
	}
}

pub use block_color::{block_color, needs_biome};

/// Grass color modifier used by a biome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeGrassColorModifier {
	/// Grass color modifier used by the dark forest biome
	DarkForest,
	/// Grass color modifier used by swamp biomes
	Swamp,
}

/// A biome specification
///
/// A Biome contains all information about a biome necessary to compute a block
/// color given a block type and depth
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Biome {
	/// Temperature value
	///
	/// For more efficient storage, the temperature is stored as an integer
	/// after mutiplying the raw value by 20
	pub temp: i8,
	/// Downfall value
	///
	/// For more efficient storage, the downfall is stored as an integer
	/// after mutiplying the raw value by 20
	pub downfall: i8,
	/// Water color override
	pub water_color: Option<Color>,
	/// Foliage color override
	pub foliage_color: Option<Color>,
	/// Grass color override
	pub grass_color: Option<Color>,
	/// Grass color modifier
	pub grass_color_modifier: Option<BiomeGrassColorModifier>,
}

impl Biome {
	/// Constructs a new Biome
	const fn new(temp: i16, downfall: i16) -> Biome {
		/// Helper to encode temperature and downfall values
		///
		/// Converts temperatue and downfall from the input format
		/// (mutiplied by 100) to i8 range for more efficient storage.
		const fn encode(v: i16) -> i8 {
			(v / 5) as i8
		}
		Biome {
			temp: encode(temp),
			downfall: encode(downfall),
			grass_color_modifier: None,
			water_color: None,
			foliage_color: None,
			grass_color: None,
		}
	}

	/// Builder function to override the biome water color
	const fn water(self, water_color: [u8; 3]) -> Biome {
		Biome {
			water_color: Some(Color(water_color)),
			..self
		}
	}

	/// Builder function to override the biome foliage color
	const fn foliage(self, foliage_color: [u8; 3]) -> Biome {
		Biome {
			foliage_color: Some(Color(foliage_color)),
			..self
		}
	}

	/// Builder function to override the biome grass color
	const fn grass(self, grass_color: [u8; 3]) -> Biome {
		Biome {
			grass_color: Some(Color(grass_color)),
			..self
		}
	}

	/// Builder function to set a grass color modifier
	const fn modify(self, grass_color_modifier: BiomeGrassColorModifier) -> Biome {
		Biome {
			grass_color_modifier: Some(grass_color_modifier),
			..self
		}
	}

	/// Decodes a temperature or downfall value from the storage format to
	/// f32 for further calculation
	fn decode(val: i8) -> f32 {
		f32::from(val) / 20.0
	}

	/// Returns the biome's temperature decoded to its original float value
	pub fn temp(&self) -> f32 {
		Self::decode(self.temp)
	}

	/// Returns the biome's downfall decoded to its original float value
	pub fn downfall(&self) -> f32 {
		Self::decode(self.downfall)
	}
}

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

		for &(old, new) in legacy_biomes::BIOME_ALIASES.iter().rev() {
			let biome = biome_map
				.get(new)
				.copied()
				.expect("Biome alias for unknown biome");
			assert!(biome_map.insert(String::from(old), biome).is_none());
		}

		let legacy_biomes = (0..=255)
			.map(|index| {
				let id = legacy_biomes::legacy_biome(index);
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
