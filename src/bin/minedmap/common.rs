use std::{
	fmt::Debug,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use minedmap::{types::*, world::layer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileCoords {
	pub x: i32,
	pub z: i32,
}

impl Debug for TileCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.z)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedChunk {
	pub blocks: Box<layer::BlockArray>,
	pub biomes: Box<layer::BiomeArray>,
	pub depths: Box<layer::DepthArray>,
}
pub type ProcessedRegion = ChunkArray<Option<ProcessedChunk>>;

pub struct Config {
	pub region_dir: PathBuf,
	pub processed_dir: PathBuf,
	pub light_dir: PathBuf,
	pub map_dir: PathBuf,
}

fn coord_filename(coords: TileCoords, ext: &str) -> String {
	format!("r.{}.{}.{}", coords.x, coords.z, ext)
}

impl Config {
	pub fn new(args: super::Args) -> Self {
		let region_dir = [&args.input_dir, Path::new("region")].iter().collect();
		let processed_dir = [&args.output_dir, Path::new("processed")].iter().collect();
		let light_dir = [&args.output_dir, Path::new("light/0")].iter().collect();
		let map_dir = [&args.output_dir, Path::new("map/0")].iter().collect();

		Config {
			region_dir,
			processed_dir,
			light_dir,
			map_dir,
		}
	}

	pub fn processed_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "bin");
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	pub fn light_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "png");
		[&self.light_dir, Path::new(&filename)].iter().collect()
	}

	pub fn map_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "png");
		[&self.map_dir, Path::new(&filename)].iter().collect()
	}
}

pub fn overlay_chunk<I, J>(image: &mut I, chunk: &J, coords: ChunkCoords)
where
	I: image::GenericImage,
	J: image::GenericImageView<Pixel = I::Pixel>,
{
	image::imageops::overlay(
		image,
		chunk,
		coords.x.0 as i64 * BLOCKS_PER_CHUNK as i64,
		coords.z.0 as i64 * BLOCKS_PER_CHUNK as i64,
	);
}
