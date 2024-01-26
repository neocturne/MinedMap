//! The [MetadataWriter] and related types

use anyhow::{Context, Result};
use regex::Regex;
use serde::Serialize;

use crate::{
	core::common::*,
	io::{fs, storage},
	world::{
		block_entity::{self, BlockEntity, BlockEntityData},
		de, sign,
	},
};

/// Minimum and maximum X and Z tile coordinates for a mipmap level
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bounds {
	/// Minimum X coordinate
	min_x: i32,
	/// Maximum X coordinate
	max_x: i32,
	/// Minimum Z coordinate
	min_z: i32,
	/// Maximum Z coordinate
	max_z: i32,
}

/// Mipmap level information in viewer metadata file
#[derive(Debug, Serialize)]
struct Mipmap<'t> {
	/// Minimum and maximum tile coordinates of the mipmap level
	bounds: Bounds,
	/// Map of populated tiles for the mipmap level
	regions: &'t TileCoordMap,
}

/// Initial spawn point for new players
#[derive(Debug, Serialize)]
struct Spawn {
	/// Spawn X coordinate
	x: i32,
	/// Spawn Z coordinate
	z: i32,
}

/// Keeps track of enabled MinedMap features
#[derive(Debug, Serialize)]
struct Features {
	/// Sign layer
	signs: bool,
}

/// Viewer metadata JSON data structure
#[derive(Debug, Serialize)]
struct Metadata<'t> {
	/// Tile information for each mipmap level
	mipmaps: Vec<Mipmap<'t>>,
	/// Initial spawn point for new players
	spawn: Spawn,
	/// Enabled MinedMap features
	features: Features,
}

/// Viewer entity JSON data structure
#[derive(Debug, Serialize, Default)]
struct Entities {
	/// List of signs
	signs: Vec<BlockEntity>,
}

/// The MetadataWriter is used to generate the viewer metadata file
pub struct MetadataWriter<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// Map of generated tiles for each mipmap level
	tiles: &'a [TileCoordMap],
}

impl<'a> MetadataWriter<'a> {
	/// Creates a new MetadataWriter
	pub fn new(config: &'a Config, tiles: &'a [TileCoordMap]) -> Self {
		MetadataWriter { config, tiles }
	}

	/// Helper to construct a [Mipmap] data structure from a [TileCoordMap]
	fn mipmap_entry(regions: &TileCoordMap) -> Mipmap {
		let mut min_x = i32::MAX;
		let mut max_x = i32::MIN;
		let mut min_z = i32::MAX;
		let mut max_z = i32::MIN;

		for (&z, xs) in &regions.0 {
			if z < min_z {
				min_z = z;
			}
			if z > max_z {
				max_z = z;
			}

			for &x in xs {
				if x < min_x {
					min_x = x;
				}
				if x > max_x {
					max_x = x;
				}
			}
		}

		Mipmap {
			bounds: Bounds {
				min_x,
				max_x,
				min_z,
				max_z,
			},
			regions,
		}
	}

	/// Reads and deserializes the `level.dat` of the Minecraft save data
	fn read_level_dat(&self) -> Result<de::LevelDat> {
		crate::nbt::data::from_file(&self.config.level_dat_path).context("Failed to read level.dat")
	}

	/// Generates [Spawn] data from a [de::LevelDat]
	fn spawn(level_dat: &de::LevelDat) -> Spawn {
		Spawn {
			x: level_dat.data.spawn_x,
			z: level_dat.data.spawn_z,
		}
	}

	/// Filter signs according to the sign pattern configuration
	fn sign_filter(&self, sign: &block_entity::Sign) -> bool {
		let front_text = sign.front_text.to_string();
		if self.config.sign_patterns.is_match(front_text.trim()) {
			return true;
		}
		let back_text = sign.back_text.to_string();
		if self.config.sign_patterns.is_match(back_text.trim()) {
			return true;
		}
		false
	}

	/// Applies a single transform to a [sign::SignText]
	///
	/// The regular expression is applied for each line of the sign text
	/// separately (actually for each element when JSON text is used)
	fn sign_text_transform(sign_text: &mut sign::SignText, transform: &(Regex, String)) {
		let (regexp, replacement) = transform;

		for line in &mut sign_text.0 {
			for text in &mut line.0 {
				text.text = regexp.replace_all(&text.text, replacement).into_owned()
			}
		}
	}

	/// Applies the configured transforms to the text of a sign
	fn sign_transform(&self, sign: &mut block_entity::Sign) {
		for transform in &self.config.sign_transforms {
			Self::sign_text_transform(&mut sign.front_text, transform);
			Self::sign_text_transform(&mut sign.back_text, transform);
		}
	}

	/// Generates [Entities] data from collected entity lists
	fn entities(&self) -> Result<Entities> {
		let data: ProcessedEntities =
			storage::read_file(&self.config.entities_path_final, storage::Format::Json)
				.context("Failed to read entity data file")?;

		let ret = Entities {
			signs: data
				.block_entities
				.into_iter()
				.filter(|entity| match &entity.data {
					BlockEntityData::Sign(sign) => self.sign_filter(sign),
				})
				.map(|mut entity| {
					match &mut entity.data {
						BlockEntityData::Sign(sign) => self.sign_transform(sign),
					};
					entity
				})
				.collect(),
		};

		Ok(ret)
	}

	/// Runs the viewer metadata file generation
	pub fn run(self) -> Result<()> {
		let level_dat = self.read_level_dat()?;

		let features = Features {
			signs: !self.config.sign_patterns.is_empty(),
		};

		let mut metadata = Metadata {
			mipmaps: Vec::new(),
			spawn: Self::spawn(&level_dat),
			features,
		};

		for tile_map in self.tiles.iter() {
			metadata.mipmaps.push(Self::mipmap_entry(tile_map));
		}

		fs::create_with_tmpfile(&self.config.viewer_info_path, |file| {
			serde_json::to_writer(file, &metadata).context("Failed to write info.json")
		})?;

		let entities = self.entities()?;
		fs::create_with_tmpfile(&self.config.viewer_entities_path, |file| {
			serde_json::to_writer(file, &entities).context("Failed to write entities.json")
		})?;

		Ok(())
	}
}
