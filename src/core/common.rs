//! Common data types and functions used by multiple generation steps

use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::Debug,
	hash::Hash,
	path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use bincode::{Decode, Encode};
use clap::ValueEnum;
use regex::{Regex, RegexSet};
use serde::Serialize;

use crate::{
	io::fs::FileMetaVersion,
	resource::Biome,
	types::*,
	world::{block_entity::BlockEntity, layer},
};

// Increase to force regeneration of all output files

/// MinedMap processed region data version number
///
/// Increase when the generation of processed regions from region data changes
/// (usually because of updated resource data)
pub const REGION_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(7);

/// MinedMap map tile data version number
///
/// Increase when the generation of map tiles from processed regions changes
/// (because of code changes in tile generation)
pub const MAP_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(0);

/// MinedMap lightmap data version number
///
/// Increase when the generation of lightmap tiles from region data changes
/// (usually because of updated resource data)
pub const LIGHTMAP_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(5);

/// MinedMap mipmap data version number
///
/// Increase when the mipmap generation changes (this should not happen)
pub const MIPMAP_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(0);

/// MinedMap processed entity data version number
///
/// Increase when entity collection changes bacause of code changes.
pub const ENTITIES_FILE_META_VERSION: FileMetaVersion = FileMetaVersion(3);

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
#[derive(Debug, Encode, Decode)]
pub struct ProcessedChunk {
	/// Block type data
	pub blocks: Box<layer::BlockArray>,
	/// Biome data
	pub biomes: Box<layer::BiomeArray>,
	/// Block height/depth data
	pub depths: Box<layer::DepthArray>,
}

/// Data structure for storing region data between processing and rendering steps
#[derive(Debug, Default, Encode, Decode)]
pub struct ProcessedRegion {
	/// List of biomes used in the region
	///
	/// Indexed by [ProcessedChunk] biome data
	pub biome_list: Vec<Biome>,
	/// Processed chunk data
	pub chunks: ChunkArray<Option<Box<ProcessedChunk>>>,
}

/// Data structure for storing entity data between processing and collection steps
#[derive(Debug, Default, Encode, Decode)]
pub struct ProcessedEntities {
	/// List of block entities
	pub block_entities: Vec<BlockEntity>,
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
#[derive(Debug)]
pub struct Config {
	/// Number of threads for parallel processing
	pub num_threads: usize,
	/// Number of threads for initial parallel processing
	pub num_threads_initial: usize,
	/// Path of input region directory
	pub region_dir: PathBuf,
	/// Path of input `level.dat` file
	pub level_dat_path: PathBuf,
	/// Path of input `level.dat_old` file
	pub level_dat_old_path: PathBuf,
	/// Base path for storage of rendered tile data
	pub output_dir: PathBuf,
	/// Path for storage of intermediate processed data files
	pub processed_dir: PathBuf,
	/// Path for storage of processed entity data files
	pub entities_dir: PathBuf,
	/// Path for storage of the final merged processed entity data file
	pub entities_path_final: PathBuf,
	/// Path of viewer metadata file
	pub viewer_info_path: PathBuf,
	/// Path of viewer entities file
	pub viewer_entities_path: PathBuf,
	/// Format of generated map tiles
	pub image_format: ImageFormat,
	/// Sign text filter patterns
	pub sign_patterns: RegexSet,
	/// Sign text transformation pattern
	pub sign_transforms: Vec<(Regex, String)>,
}

impl Config {
	/// Crates a new [Config] from [command line arguments](super::Args)
	pub fn new(args: &super::Args) -> Result<Self> {
		let num_threads = match args.jobs {
			Some(0) => num_cpus::get(),
			Some(threads) => threads,
			None => 1,
		};
		let num_threads_initial = args.jobs_initial.unwrap_or(num_threads);

		let region_dir = [&args.input_dir, Path::new("region")].iter().collect();
		let level_dat_path = [&args.input_dir, Path::new("level.dat")].iter().collect();
		let level_dat_old_path = [&args.input_dir, Path::new("level.dat_old")]
			.iter()
			.collect();
		let processed_dir: PathBuf = [&args.output_dir, Path::new("processed")].iter().collect();
		let entities_dir: PathBuf = [&processed_dir, Path::new("entities")].iter().collect();
		let entities_path_final = [&entities_dir, Path::new("entities.bin")].iter().collect();
		let viewer_info_path = [&args.output_dir, Path::new("info.json")].iter().collect();
		let viewer_entities_path = [&args.output_dir, Path::new("entities.json")]
			.iter()
			.collect();

		let sign_patterns = Self::sign_patterns(args).context("Failed to parse sign patterns")?;
		let sign_transforms =
			Self::sign_transforms(args).context("Failed to parse sign transforms")?;

		Ok(Config {
			num_threads,
			num_threads_initial,
			region_dir,
			level_dat_path,
			level_dat_old_path,
			output_dir: args.output_dir.clone(),
			processed_dir,
			entities_dir,
			entities_path_final,
			viewer_info_path,
			viewer_entities_path,
			image_format: args.image_format,
			sign_patterns,
			sign_transforms,
		})
	}

	/// Parses the sign prefixes and sign filters into a [RegexSet]
	fn sign_patterns(args: &super::Args) -> Result<RegexSet> {
		let prefix_patterns: Vec<_> = args
			.sign_prefix
			.iter()
			.map(|prefix| format!("^{}", regex::escape(prefix)))
			.collect();
		Ok(RegexSet::new(
			prefix_patterns.iter().chain(args.sign_filter.iter()),
		)?)
	}

	/// Parses the sign transform argument into a vector of [Regex] and
	/// corresponding replacement strings
	fn sign_transforms(args: &super::Args) -> Result<Vec<(Regex, String)>> {
		let splitter = Regex::new(r"^s/((?:[^\\/]|\\.)*)/((?:[^\\/]|\\.)*)/$").unwrap();

		args.sign_transform
			.iter()
			.map(|t| Self::sign_transform(&splitter, t))
			.collect()
	}

	/// Parses the sign transform argument into a [Regex] and its corresponding
	/// replacement string
	fn sign_transform(splitter: &Regex, transform: &str) -> Result<(Regex, String)> {
		let captures = splitter
			.captures(transform)
			.with_context(|| format!("Invalid transform pattern '{transform}'"))?;
		let regexp = Regex::new(&captures[1])?;
		let replacement = captures[2].to_string();
		Ok((regexp, replacement))
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

	/// Constructs the base output path for processed entity data
	pub fn entities_dir(&self, level: usize) -> PathBuf {
		[&self.entities_dir, Path::new(&level.to_string())]
			.iter()
			.collect()
	}

	/// Constructs the path of a processed entity data file
	pub fn entities_path(&self, level: usize, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, "bin");
		let dir = self.entities_dir(level);
		[Path::new(&dir), Path::new(&filename)].iter().collect()
	}

	/// Constructs the base output path for a [TileKind] and mipmap level
	pub fn tile_dir(&self, kind: TileKind, level: usize) -> PathBuf {
		let prefix = match kind {
			TileKind::Map => "map",
			TileKind::Lightmap => "light",
		};
		let dir = format!("{prefix}/{level}");
		[&self.output_dir, Path::new(&dir)].iter().collect()
	}

	/// Returns the file extension for the configured image format
	pub fn tile_extension(&self) -> &'static str {
		match self.image_format {
			ImageFormat::Png => "png",
			ImageFormat::Webp => "webp",
		}
	}
	/// Returns the configurured image format for the image library
	pub fn tile_image_format(&self) -> image::ImageFormat {
		match self.image_format {
			ImageFormat::Png => image::ImageFormat::Png,
			ImageFormat::Webp => image::ImageFormat::WebP,
		}
	}

	/// Constructs the path of an output tile image
	pub fn tile_path(&self, kind: TileKind, level: usize, coords: TileCoords) -> PathBuf {
		let filename = coord_filename(coords, self.tile_extension());
		let dir = self.tile_dir(kind, level);
		[Path::new(&dir), Path::new(&filename)].iter().collect()
	}
}

/// Format of generated map tiles
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ImageFormat {
	/// Generate PNG images
	#[default]
	Png,
	/// Generate WebP images
	Webp,
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
