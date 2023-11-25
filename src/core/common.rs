//! Common data types and functions used by multiple generation steps

use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::Debug,
	path::{Path, PathBuf},
};

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{io::fs::FileMetaVersion, resource::Biome, types::*, world::layer};

/// Increase to force regeneration of all output files

/// MinedMap processed region data version number
pub const REGION_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(0);

/// MinedMap map tile data version number
pub const MAP_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(0);

/// MinedMap lightmap data version number
pub const LIGHTMAP_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(0);

/// Coordinate pair of a generated tile
///
/// Each tile corresponds to one Minecraft region file
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileCoords {
	/// The X coordinate
	pub x: i32,
	/// The Z coordinate
	pub z: i32,
}

impl Debug for TileCoords {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.z)
	}
}

/// Set of tile coordinates
///
/// Used to store list of populated tiles for each mipmap level in the
/// viewer metadata file.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct TileCoordMap(pub BTreeMap<i32, BTreeSet<i32>>);

impl TileCoordMap {
	/// Checks whether the map contains a given coordinate pair
	pub fn contains(&self, coords: TileCoords) -> bool {
		let Some(xs) = self.0.get(&coords.z) else {
			return false;
		};

		xs.contains(&coords.x)
	}
}

/// Data structure for storing chunk data between processing and rendering steps
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedChunk {
	/// Block type data
	pub blocks: Box<layer::BlockArray>,
	/// Biome data
	pub biomes: Box<layer::BiomeArray>,
	/// Block height/depth data
	pub depths: Box<layer::DepthArray>,
}

/// Data structure for storing region data between processing and rendering steps
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProcessedRegion {
	/// List of biomes used in the region
	///
	/// Indexed by [ProcessedChunk] biome data
	pub biome_list: IndexSet<Biome>,
	/// Processed chunk data
	pub chunks: ChunkArray<Option<Box<ProcessedChunk>>>,
}

/// Derives a filename from region coordinates and a file extension
///
/// Can be used for input regions, processed data or rendered tiles
fn coord_filename(coords: TileCoords, ext: &str) -> String {
	format!("r.{}.{}.{}", coords.x, coords.z, ext)
}

/// Tile kind corresponding to a map layer
#[derive(Debug, Clone, Copy)]
pub enum TileKind {
	/// Regular map tile contains block colors
	Map,
	/// Lightmap tile for illumination layer
	Lightmap,
}

/// Common configuration based on command line arguments
pub struct Config {
	/// Number of threads for parallel processing
	pub num_threads: usize,
	/// Path of input region directory
	pub region_dir: PathBuf,
	/// Path of input `level.dat` file
	pub level_dat_path: PathBuf,
	/// Base path for storage of rendered tile data
	pub output_dir: PathBuf,
	/// Path for storage of intermediate processed data files
	pub processed_dir: PathBuf,
	/// Path of viewer metadata file
	pub metadata_path: PathBuf,
}

impl Config {
	/// Crates a new [Config] from [command line arguments](super::Args)
	pub fn new(args: &super::Args) -> Self {
		let num_threads = match args.jobs {
			Some(0) => num_cpus::get(),
			Some(threads) => threads,
			None => 1,
		};

		let region_dir = [&args.input_dir, Path::new("region")].iter().collect();
		let level_dat_path = [&args.input_dir, Path::new("level.dat")].iter().collect();
		let processed_dir = [&args.output_dir, Path::new("processed")].iter().collect();
		let metadata_path = [&args.output_dir, Path::new("info.json")].iter().collect();

		Config {
			num_threads,
			region_dir,
			level_dat_path,
			output_dir: args.output_dir.clone(),
			processed_dir,
			metadata_path,
		}
	}

	/// Constructs the path to an input region file
	pub fn region_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "mca");
		[&self.region_dir, Path::new(&filename)].iter().collect()
	}

	/// Constructs the path of an intermediate processed region file
	pub fn processed_path(&self, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "bin");
		[&self.processed_dir, Path::new(&filename)].iter().collect()
	}

	/// Constructs the base output path for a [TileKind] and mipmap level
	pub fn tile_dir(&self, kind: TileKind, level: usize) -> PathBuf {
		let prefix = match kind {
			TileKind::Map => "map",
			TileKind::Lightmap => "light",
		};
		let dir = format!("{}/{}", prefix, level);
		[&self.output_dir, Path::new(&dir)].iter().collect()
	}

	/// Constructs the path of an output tile image
	pub fn tile_path(&self, kind: TileKind, level: usize, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "png");
		let dir = self.tile_dir(kind, level);
		[Path::new(&dir), Path::new(&filename)].iter().collect()
	}
}

/// Copies a chunk image into a region tile
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
