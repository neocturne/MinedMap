//! The [TileMipmapper]

use std::ops::Add;

use anyhow::{Context, Result};
use tracing::{debug, info, warn};

use super::{common::*, tile_collector::TileCollector};
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

impl MipmapStat {
	/// Mipmap step return when none of the input files exist
	const NOT_FOUND: MipmapStat = MipmapStat {
		total: 0,
		processed: 0,
	};
	/// Mipmap step return when output file is up-to-date
	const SKIPPED: MipmapStat = MipmapStat {
		total: 1,
		processed: 0,
	};
	/// Mipmap step return when a new output file has been generated
	const PROCESSED: MipmapStat = MipmapStat {
		total: 1,
		processed: 1,
	};
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

/// Generates mipmap tiles from full-resolution tile images
pub struct TileMipmapper<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// List of populated tiles for base mipmap level (level 0)
	regions: &'a [TileCoords],
}

impl<'a> TileCollector for TileMipmapper<'a> {
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
		/// Tile width/height
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let output_path = self.config.tile_path(kind, level, coords);
		let output_timestamp = fs::read_timestamp(&output_path, MIPMAP_FILE_META_VERSION);

		let sources: Vec<_> = [(0, 0), (0, 1), (1, 0), (1, 1)]
			.into_iter()
			.filter_map(|(dx, dz)| {
				let source_coords = TileCoords {
					x: 2 * coords.x + dx,
					z: 2 * coords.z + dz,
				};
				if !prev.contains(source_coords) {
					return None;
				}

				let source_path = self.config.tile_path(kind, level - 1, source_coords);
				let timestamp = match fs::modified_timestamp(&source_path) {
					Ok(timestamp) => timestamp,
					Err(err) => {
						warn!("{}", err);
						return None;
					}
				};
				Some(((dx, dz), source_path, timestamp))
			})
			.collect();

		let Some(input_timestamp) = sources.iter().map(|(_, _, ts)| *ts).max() else {
			return Ok(MipmapStat::NOT_FOUND);
		};

		if Some(input_timestamp) <= output_timestamp {
			debug!(
				"Skipping unchanged mipmap tile {}",
				output_path
					.strip_prefix(&self.config.output_dir)
					.expect("tile path must be in output directory")
					.display(),
			);
			return Ok(MipmapStat::SKIPPED);
		}

		debug!(
			"Rendering mipmap tile {}",
			output_path
				.strip_prefix(&self.config.output_dir)
				.expect("tile path must be in output directory")
				.display(),
		);

		let mut image: image::DynamicImage =
			image::ImageBuffer::<P, Vec<P::Subpixel>>::new(N, N).into();

		for ((dx, dz), source_path, _) in sources {
			let source = match image::open(&source_path) {
				Ok(source) => source,
				Err(err) => {
					warn!(
						"Failed to read source image {}: {}",
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
				dx as i64 * (N / 2) as i64,
				dz as i64 * (N / 2) as i64,
			);
		}

		fs::create_with_timestamp(
			&output_path,
			MIPMAP_FILE_META_VERSION,
			input_timestamp,
			|file| {
				image
					.write_to(file, image::ImageFormat::Png)
					.context("Failed to save image")
			},
		)?;

		Ok(MipmapStat::PROCESSED)
	}

	/// Runs the mipmap generation
	pub fn run(self) -> Result<Vec<TileCoordMap>> {
		self.collect_tiles()
	}
}
