//! The [TileMipmapper]

use std::sync::mpsc;

use anyhow::{Context, Result};
use rayon::prelude::*;
use tracing::{debug, info, warn};

use super::common::*;
use crate::{io::fs, types::*};

/// Generates mipmap tiles from full-resolution tile images
pub struct TileMipmapper<'a> {
	/// Common MinedMap configuration from command line
	config: &'a Config,
	/// List of populated tiles for base mipmap level (level 0)
	regions: &'a [TileCoords],
}

impl<'a> TileMipmapper<'a> {
	/// Constructs a new TileMipmapper
	pub fn new(config: &'a Config, regions: &'a [TileCoords]) -> Self {
		TileMipmapper { config, regions }
	}

	/// Helper to determine if no further mipmap levels are needed
	///
	/// If all tile coordinates are -1 or 0, further mipmap levels will not
	/// decrease the number of tiles and mipmap generated is considered finished.
	fn done(tiles: &TileCoordMap) -> bool {
		tiles
			.0
			.iter()
			.all(|(z, xs)| (-1..=0).contains(z) && xs.iter().all(|x| (-1..=0).contains(x)))
	}

	/// Derives the map of populated tile coordinates for the next mipmap level
	fn map_coords(tiles: &TileCoordMap) -> TileCoordMap {
		let mut ret = TileCoordMap::default();

		for (&z, xs) in &tiles.0 {
			for &x in xs {
				let xt = x >> 1;
				let zt = z >> 1;

				ret.0.entry(zt).or_default().insert(xt);
			}
		}

		ret
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
		count_total: &mpsc::Sender<()>,
		count_processed: &mpsc::Sender<()>,
	) -> Result<()>
	where
		[P::Subpixel]: image::EncodableLayout,
		image::ImageBuffer<P, Vec<P::Subpixel>>: Into<image::DynamicImage>,
	{
		/// Tile width/height
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let version = match kind {
			TileKind::Map => REGION_FILE_META_VERSION,
			TileKind::Lightmap => LIGHTMAP_FILE_META_VERSION,
		};
		let output_path = self.config.tile_path(kind, level, coords);
		let output_timestamp = fs::read_timestamp(&output_path, version);

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
			return Ok(());
		};

		count_total.send(()).unwrap();

		if Some(input_timestamp) <= output_timestamp {
			debug!(
				"Skipping unchanged mipmap tile {}",
				output_path
					.strip_prefix(&self.config.output_dir)
					.expect("tile path must be in output directory")
					.display(),
			);
			return Ok(());
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

		fs::create_with_timestamp(&output_path, version, input_timestamp, |file| {
			image
				.write_to(file, image::ImageFormat::Png)
				.context("Failed to save image")
		})?;

		count_processed.send(()).unwrap();
		Ok(())
	}

	/// Runs the mipmap generation
	pub fn run(self) -> Result<Vec<TileCoordMap>> {
		let mut tile_stack = {
			let mut tile_map = TileCoordMap::default();

			for &TileCoords { x, z } in self.regions {
				tile_map.0.entry(z).or_default().insert(x);
			}

			vec![tile_map]
		};

		loop {
			let level = tile_stack.len();
			let prev = &tile_stack[level - 1];
			if Self::done(prev) {
				break;
			}

			info!("Generating level {} mipmaps...", level);

			fs::create_dir_all(&self.config.tile_dir(TileKind::Map, level))?;
			fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, level))?;

			let next = Self::map_coords(prev);

			let (total_send, total_recv) = mpsc::channel();
			let (processed_send, processed_recv) = mpsc::channel();

			next.0
				.par_iter()
				.flat_map(|(&z, xs)| xs.par_iter().map(move |&x| TileCoords { x, z }))
				.try_for_each(|coords| {
					self.render_mipmap::<image::Rgba<u8>>(
						TileKind::Map,
						level,
						coords,
						prev,
						&total_send,
						&processed_send,
					)?;
					self.render_mipmap::<image::LumaA<u8>>(
						TileKind::Lightmap,
						level,
						coords,
						prev,
						&total_send,
						&processed_send,
					)?;

					anyhow::Ok(())
				})?;

			drop(total_send);
			let total = total_recv.into_iter().count();

			drop(processed_send);
			let processed = processed_recv.into_iter().count();

			info!(
				"Generated level {} mipmaps ({} processed, {} unchanged)",
				level,
				processed,
				total - processed,
			);

			tile_stack.push(next);
		}

		Ok(tile_stack)
	}
}
