use std::{
	collections::{BTreeMap, BTreeSet},
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct TileCoordMap(pub BTreeMap<i32, BTreeSet<i32>>);

impl TileCoordMap {
	pub fn contains(&self, coords: TileCoords) -> bool {
		let Some(xs) = self.0.get(&coords.z) else {
			return false;
		};

		xs.contains(&coords.x)
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
	pub output_dir: PathBuf,
	pub level_dat_path: PathBuf,
	pub metadata_path: PathBuf,
}

fn coord_filename(coords: TileCoords, ext: &str) -> String {
	format!("r.{}.{}.{}", coords.x, coords.z, ext)
}

#[derive(Debug, Clone, Copy)]
pub enum TileKind {
	Map,
	Lightmap,
}

impl Config {
	pub fn new(args: super::Args) -> Self {
		let region_dir = [&args.input_dir, Path::new("region")].iter().collect();
		let processed_dir = [&args.output_dir, Path::new("processed")].iter().collect();
		let level_dat_path = [&args.input_dir, Path::new("level.dat")].iter().collect();
		let metadata_path = [&args.output_dir, Path::new("info.json")].iter().collect();

		Config {
			region_dir,
			processed_dir,
			output_dir: args.output_dir,
			level_dat_path,
			metadata_path,
		}
	}

	pub fn processed_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "bin");
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	pub fn tile_dir(&self, kind: TileKind, level: usize) -> PathBuf {
		let prefix = match kind {
			TileKind::Map => "map",
			TileKind::Lightmap => "light",
		};
		let dir = format!("{}/{}", prefix, level);
		[&self.output_dir, Path::new(&dir)].iter().collect()
	}

	pub fn tile_path(&self, kind: TileKind, level: usize, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "png");
		let dir = self.tile_dir(kind, level);
		[Path::new(&dir), Path::new(&filename)].iter().collect()
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
