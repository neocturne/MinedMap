use anyhow::{Context, Result};
use minedmap::{io::fs, world::de};
use serde::Serialize;

use super::common::*;

pub struct MetadataWriter<'a> {
	config: &'a Config,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bounds {
	min_x: i32,
	max_x: i32,
	min_z: i32,
	max_z: i32,
}

#[derive(Debug, Serialize)]
struct Mipmap<'t> {
	bounds: Bounds,
	regions: &'t TileCoordMap,
}

#[derive(Debug, Serialize)]
struct Spawn {
	x: i32,
	z: i32,
}

#[derive(Debug, Serialize)]
struct Metadata<'t> {
	mipmaps: Vec<Mipmap<'t>>,
	spawn: Spawn,
}

impl<'a> MetadataWriter<'a> {
	pub fn new(config: &'a Config) -> Self {
		MetadataWriter { config }
	}

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

	fn read_level_dat(&self) -> Result<de::LevelDat> {
		minedmap::io::data::from_file(&self.config.level_dat_path)
			.context("Failed to read level.dat")
	}

	fn spawn(level_dat: &de::LevelDat) -> Spawn {
		Spawn {
			x: level_dat.data.spawn_x,
			z: level_dat.data.spawn_z,
		}
	}

	pub fn run(&self, tiles: Vec<TileCoordMap>) -> Result<()> {
		let level_dat = self.read_level_dat()?;

		let mut metadata = Metadata {
			mipmaps: Vec::new(),
			spawn: Self::spawn(&level_dat),
		};

		for tile_map in tiles.iter() {
			metadata.mipmaps.push(Self::mipmap_entry(tile_map));
		}

		fs::create_with_tmpfile(&self.config.metadata_path, |file| {
			serde_json::to_writer(file, &metadata).context("Failed to write metadata")
		})
	}
}
