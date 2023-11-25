//! The [RegionProcessor] and related functions

use std::{
	ffi::OsStr,
	path::{Path, PathBuf},
	sync::mpsc,
	time::SystemTime,
};

use anyhow::{Context, Result};
use rayon::prelude::*;
use tracing::{debug, info, warn};

use super::common::*;
use crate::{
	io::{fs, storage},
	resource,
	types::*,
	world::{self, layer},
};

/// Parses a filename in the format r.X.Z.mca into the contained X and Z values
fn parse_region_filename(file_name: &OsStr) -> Option<TileCoords> {
	let parts: Vec<_> = file_name.to_str()?.split('.').collect();
	let &["r", x, z, "mca"] = parts.as_slice() else {
		return None;
	};

	Some(TileCoords {
		x: x.parse().ok()?,
		z: z.parse().ok()?,
	})
}

/// [RegionProcessor::process_region] return values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegionProcessorStatus {
	/// Region was processed
	Ok,
	/// Region was unchanged and skipped
	Skipped,
	/// Reading the region failed, previous processed data is reused
	ErrorOk,
	/// Reading the region failed, no previous data available
	ErrorMissing,
}

/// Handles processing for a single region
struct SingleRegionProcessor<'a> {
	/// Registry of known block types
	block_types: &'a resource::BlockTypes,
	/// Registry of known biome types
	biome_types: &'a resource::BiomeTypes,
	/// Coordinates of the region this instance is processing
	coords: TileCoords,
	/// Input region filename
	input_path: PathBuf,
	/// Processed region data output filename
	output_path: PathBuf,
	/// Lightmap output filename
	lightmap_path: PathBuf,
	/// Timestamp of last modification of input file
	input_timestamp: SystemTime,
	/// Timestamp of last modification of processed region output file (if valid)
	output_timestamp: Option<SystemTime>,
	/// Timestamp of last modification of lightmap output file (if valid)
	lightmap_timestamp: Option<SystemTime>,
	/// Processed region intermediate data
	processed_region: ProcessedRegion,
	/// Lightmap intermediate data
	lightmap: image::GrayAlphaImage,
}

impl<'a> SingleRegionProcessor<'a> {
	/// Initializes a [SingleRegionProcessor]
	fn new(processor: &'a RegionProcessor<'a>, coords: TileCoords) -> Result<Self> {
		/// Width/height of the region data
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let input_path = processor.config.region_path(coords);
		let input_timestamp = fs::modified_timestamp(&input_path)?;

		let output_path = processor.config.processed_path(coords);
		let output_timestamp = fs::read_timestamp(&output_path, REGION_FILE_META_VERSION);
		let lightmap_path = processor.config.tile_path(TileKind::Lightmap, 0, coords);
		let lightmap_timestamp = fs::read_timestamp(&lightmap_path, LIGHTMAP_FILE_META_VERSION);

		let processed_region = ProcessedRegion::default();
		let lightmap = image::GrayAlphaImage::new(N, N);

		Ok(SingleRegionProcessor {
			block_types: &processor.block_types,
			biome_types: &processor.biome_types,
			coords,
			input_path,
			output_path,
			lightmap_path,
			input_timestamp,
			output_timestamp,
			lightmap_timestamp,
			processed_region,
			lightmap,
		})
	}

	/// Renders a lightmap subtile from chunk block light data
	fn render_chunk_lightmap(
		block_light: Box<world::layer::BlockLightArray>,
	) -> image::GrayAlphaImage {
		/// Width/height of generated chunk lightmap
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		image::GrayAlphaImage::from_fn(N, N, |x, z| {
			let v: f32 = block_light[LayerBlockCoords {
				x: BlockX::new(x),
				z: BlockZ::new(z),
			}]
			.into();
			image::LumaA([0, (192.0 * (1.0 - v / 15.0)) as u8])
		})
	}

	/// Saves processed region data
	///
	/// The timestamp is the time of the last modification of the input region data.
	fn save_region(
		path: &Path,
		processed_region: &ProcessedRegion,
		timestamp: SystemTime,
	) -> Result<()> {
		storage::write(path, processed_region, REGION_FILE_META_VERSION, timestamp)
	}

	/// Saves a lightmap tile
	///
	/// The timestamp is the time of the last modification of the input region data.
	fn save_lightmap(
		path: &Path,
		lightmap: &image::GrayAlphaImage,
		timestamp: SystemTime,
	) -> Result<()> {
		fs::create_with_timestamp(path, LIGHTMAP_FILE_META_VERSION, timestamp, |file| {
			lightmap
				.write_to(file, image::ImageFormat::Png)
				.context("Failed to save image")
		})
	}

	/// Processes the region
	fn run(mut self) -> Result<RegionProcessorStatus> {
		if Some(self.input_timestamp) <= self.output_timestamp
			&& Some(self.input_timestamp) <= self.lightmap_timestamp
		{
			debug!(
				"Skipping unchanged region r.{}.{}.mca",
				self.coords.x, self.coords.z
			);
			return Ok(RegionProcessorStatus::Skipped);
		}

		debug!(
			"Processing region r.{}.{}.mca",
			self.coords.x, self.coords.z
		);

		if let Err(err) = (|| -> Result<()> {
			crate::nbt::region::from_file(&self.input_path)?.foreach_chunk(
				|chunk_coords, data: world::de::Chunk| {
					let chunk = world::chunk::Chunk::new(&data, self.block_types, self.biome_types)
						.with_context(|| format!("Failed to decode chunk {:?}", chunk_coords))?;
					let Some(layer::LayerData {
						blocks,
						biomes,
						block_light,
						depths,
					}) = world::layer::top_layer(&mut self.processed_region.biome_list, &chunk)
						.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))?
					else {
						return Ok(());
					};
					self.processed_region.chunks[chunk_coords] = Some(Box::new(ProcessedChunk {
						blocks,
						biomes,
						depths,
					}));

					let chunk_lightmap = Self::render_chunk_lightmap(block_light);
					overlay_chunk(&mut self.lightmap, &chunk_lightmap, chunk_coords);

					Ok(())
				},
			)
		})() {
			if self.output_timestamp.is_some() && self.lightmap_timestamp.is_some() {
				warn!(
					"Failed to process region {:?}, using old data: {:?}",
					self.coords, err
				);
				return Ok(RegionProcessorStatus::ErrorOk);
			} else {
				warn!(
					"Failed to process region {:?}, no old data available: {:?}",
					self.coords, err
				);
				return Ok(RegionProcessorStatus::ErrorMissing);
			}
		}

		if Some(self.input_timestamp) > self.output_timestamp {
			Self::save_region(
				&self.output_path,
				&self.processed_region,
				self.input_timestamp,
			)?;
		}
		if Some(self.input_timestamp) > self.lightmap_timestamp {
			Self::save_lightmap(&self.lightmap_path, &self.lightmap, self.input_timestamp)?;
		}

		Ok(RegionProcessorStatus::Ok)
	}
}

/// Type with methods for processing the regions of a Minecraft save directory
///
/// The RegionProcessor builds lightmap tiles as well as processed region data
/// consumed by subsequent generation steps.
pub struct RegionProcessor<'a> {
	/// Registry of known block types
	block_types: resource::BlockTypes,
	/// Registry of known biome types
	biome_types: resource::BiomeTypes,
	/// Common MinedMap configuration from command line
	config: &'a Config,
}

impl<'a> RegionProcessor<'a> {
	/// Constructs a new RegionProcessor
	pub fn new(config: &'a Config) -> Self {
		RegionProcessor {
			block_types: resource::BlockTypes::default(),
			biome_types: resource::BiomeTypes::default(),
			config,
		}
	}

	/// Generates a list of all regions of the input Minecraft save data
	fn collect_regions(&self) -> Result<Vec<TileCoords>> {
		Ok(self
			.config
			.region_dir
			.read_dir()
			.with_context(|| {
				format!(
					"Failed to read directory {}",
					self.config.region_dir.display()
				)
			})?
			.filter_map(|entry| entry.ok())
			.filter(|entry| {
				// We are only interested in regular files
				matches!(
					entry.file_type().map(|file_type| file_type.is_file()),
					Ok(true)
				)
			})
			.filter_map(|entry| parse_region_filename(&entry.file_name()))
			.collect())
	}

	/// Processes a single region file
	fn process_region(&self, coords: TileCoords) -> Result<RegionProcessorStatus> {
		SingleRegionProcessor::new(self, coords)?.run()
	}

	/// Iterates over all region files of a Minecraft save directory
	///
	/// Returns a list of the coordinates of all processed regions
	pub fn run(self) -> Result<Vec<TileCoords>> {
		fs::create_dir_all(&self.config.processed_dir)?;
		fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, 0))?;

		info!("Processing region files...");

		let (region_send, region_recv) = mpsc::channel();
		let (processed_send, processed_recv) = mpsc::channel();
		let (error_send, error_recv) = mpsc::channel();

		self.collect_regions()?.par_iter().try_for_each(|&coords| {
			let ret = self
				.process_region(coords)
				.with_context(|| format!("Failed to process region {:?}", coords))?;

			if ret != RegionProcessorStatus::ErrorMissing {
				region_send.send(coords).unwrap();
			}

			match ret {
				RegionProcessorStatus::Ok => processed_send.send(()).unwrap(),
				RegionProcessorStatus::Skipped => {}
				RegionProcessorStatus::ErrorOk | RegionProcessorStatus::ErrorMissing => {
					error_send.send(()).unwrap()
				}
			}

			anyhow::Ok(())
		})?;

		drop(region_send);
		let mut regions: Vec<_> = region_recv.into_iter().collect();

		drop(processed_send);
		let processed = processed_recv.into_iter().count();
		drop(error_send);
		let errors = error_recv.into_iter().count();

		info!(
			"Processed region files ({} processed, {} unchanged, {} errors)",
			processed,
			regions.len() - processed - errors,
			errors,
		);

		// Sort regions in a zig-zag pattern to optimize cache usage
		regions.sort_unstable_by_key(|&TileCoords { x, z }| (x, if x % 2 == 0 { z } else { -z }));

		Ok(regions)
	}
}
