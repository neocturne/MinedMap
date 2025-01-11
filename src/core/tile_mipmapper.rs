//! The [TileMipmapper]

use std::{marker::PhantomData, ops::Add};

use anyhow::{Context, Result};
use tracing::{debug, info, warn};

use super::{
	common::*,
	tile_collector::TileCollector,
	tile_merger::{self, TileMerger},
};
use crate::{io::fs, types::*};

/// Counters for the number of processed and total tiles
///
/// Used as return of [TileMipmapper::collect_one]
#[derive(Debug, Clone, Copy)]
pub struct MipmapStat {
	/// Total number of tiles
	total: usize,
	/// Processed number of tiles
	processed: usize,
}

impl From<tile_merger::Stat> for MipmapStat {
	fn from(value: tile_merger::Stat) -> Self {
		match value {
			tile_merger::Stat::NotFound => MipmapStat {
				total: 0,
				processed: 0,
			},
			tile_merger::Stat::Skipped => MipmapStat {
				total: 1,
				processed: 0,
			},
			tile_merger::Stat::Regenerate => MipmapStat {
				total: 1,
				processed: 1,
			},
		}
	}
}

impl Add for MipmapStat {
	type Output = MipmapStat;

	fn add(self, rhs: Self) -> Self::Output {
		MipmapStat {
			total: self.total + rhs.total,
			processed: self.processed + rhs.processed,
		}
	}
}

/// [TileMerger] for map tile images
struct MapMerger<'a, P> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// Tile kind (map or lightmap)
	kind: TileKind,
	/// Pixel format type
	_pixel: PhantomData<P>,
}

impl<'a, P> MapMerger<'a, P> {
	/// Creates a new [MapMerger]
	fn new(config: &'a Config, kind: TileKind) -> Self {
		MapMerger {
			config,
			kind,
			_pixel: PhantomData,
		}
	}
}

impl<P: image::PixelWithColorType> TileMerger for MapMerger<'_, P>
where
	[P::Subpixel]: image::EncodableLayout,
	image::ImageBuffer<P, Vec<P::Subpixel>>: Into<image::DynamicImage>,
{
	fn file_meta_version(&self) -> fs::FileMetaVersion {
		MIPMAP_FILE_META_VERSION
	}

	fn tile_path(&self, level: usize, coords: TileCoords) -> std::path::PathBuf {
		self.config.tile_path(self.kind, level, coords)
	}

	fn log(&self, output_path: &std::path::Path, stat: super::tile_merger::Stat) {
		match stat {
			super::tile_merger::Stat::NotFound => {}
			super::tile_merger::Stat::Skipped => {
				debug!(
					"Skipping unchanged mipmap tile {}",
					output_path
						.strip_prefix(&self.config.output_dir)
						.expect("tile path must be in output directory")
						.display(),
				);
			}
			super::tile_merger::Stat::Regenerate => {
				debug!(
					"Rendering mipmap tile {}",
					output_path
						.strip_prefix(&self.config.output_dir)
						.expect("tile path must be in output directory")
						.display(),
				);
			}
		};
	}

	fn write_tile(
		&self,
		file: &mut std::io::BufWriter<std::fs::File>,
		sources: &[super::tile_merger::Source],
	) -> Result<()> {
		/// Tile width/height
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let mut image: image::DynamicImage =
			image::ImageBuffer::<P, Vec<P::Subpixel>>::new(N, N).into();

		for ((dx, dz), source_path, _) in sources {
			let source = match image::open(source_path) {
				Ok(source) => source,
				Err(err) => {
					warn!(
						"Failed to read source image {}: {:?}",
						source_path.display(),
						err,
					);
					continue;
				}
			};
			let resized = source.resize(N / 2, N / 2, image::imageops::FilterType::Triangle);
			image::imageops::overlay(
				&mut image,
				&resized,
				*dx as i64 * (N / 2) as i64,
				*dz as i64 * (N / 2) as i64,
			);
		}

		image
			.write_to(file, self.config.tile_image_format())
			.context("Failed to save image")
	}
}

/// Generates mipmap tiles from full-resolution tile images
pub struct TileMipmapper<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// List of populated tiles for base mipmap level (level 0)
	regions: &'a [TileCoords],
}

impl TileCollector for TileMipmapper<'_> {
	type CollectOutput = MipmapStat;

	fn tiles(&self) -> &[TileCoords] {
		self.regions
	}

	fn prepare(&self, level: usize) -> Result<()> {
		info!("Generating level {} mipmaps...", level);

		fs::create_dir_all(&self.config.tile_dir(TileKind::Map, level))?;
		fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, level))?;

		Ok(())
	}

	fn finish(
		&self,
		level: usize,
		outputs: impl Iterator<Item = Self::CollectOutput>,
	) -> Result<()> {
		let stat = outputs.fold(
			MipmapStat {
				total: 0,
				processed: 0,
			},
			MipmapStat::add,
		);
		info!(
			"Generated level {} mipmaps ({} processed, {} unchanged)",
			level,
			stat.processed,
			stat.total - stat.processed,
		);

		Ok(())
	}

	fn collect_one(
		&self,
		level: usize,
		coords: TileCoords,
		prev: &TileCoordMap,
	) -> Result<Self::CollectOutput> {
		let map_stat = self.render_mipmap::<image::Rgba<u8>>(TileKind::Map, level, coords, prev)?;
		let lightmap_stat =
			self.render_mipmap::<image::LumaA<u8>>(TileKind::Lightmap, level, coords, prev)?;
		Ok(map_stat + lightmap_stat)
	}
}

impl<'a> TileMipmapper<'a> {
	/// Constructs a new TileMipmapper
	pub fn new(config: &'a Config, regions: &'a [TileCoords]) -> Self {
		TileMipmapper { config, regions }
	}

	/// Renders and saves a single mipmap tile image
	///
	/// Each mipmap tile is rendered by taking 2x2 tiles from the
	/// previous level and scaling them down by 50%.
	fn render_mipmap<P: image::PixelWithColorType>(
		&self,
		kind: TileKind,
		level: usize,
		coords: TileCoords,
		prev: &TileCoordMap,
	) -> Result<MipmapStat>
	where
		[P::Subpixel]: image::EncodableLayout,
		image::ImageBuffer<P, Vec<P::Subpixel>>: Into<image::DynamicImage>,
	{
		let merger = MapMerger::<P>::new(self.config, kind);
		let ret = merger.merge_tiles(level, coords, prev)?;
		Ok(ret.into())
	}

	/// Runs the mipmap generation
	pub fn run(self) -> Result<Vec<TileCoordMap>> {
		self.collect_tiles()
	}
}
