//! The [RegionProcessor] and related functions

use std::{ffi::OsStr, path::Path, time::SystemTime};

use anyhow::{Context, Result};
use indexmap::IndexSet;
use rayon::prelude::*;
use tracing::{debug, error, info};

use super::common::*;
use crate::{
	io::{fs, storage},
	resource::{self, Biome},
	types::*,
	world::{
		self,
		layer::{self, LayerData},
	},
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

	/// Processes a single chunk
	fn process_chunk(
		&self,
		biome_list: &mut IndexSet<Biome>,
		data: world::de::Chunk,
	) -> Result<Option<LayerData>> {
		let chunk = world::chunk::Chunk::new(&data, &self.block_types, &self.biome_types)?;
		world::layer::top_layer(biome_list, &chunk)
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
		storage::write(path, processed_region, FILE_META_VERSION, timestamp)
	}

	/// Saves a lightmap tile
	///
	/// The timestamp is the time of the last modification of the input region data.
	fn save_lightmap(
		path: &Path,
		lightmap: &image::GrayAlphaImage,
		timestamp: SystemTime,
	) -> Result<()> {
		fs::create_with_timestamp(path, FILE_META_VERSION, timestamp, |file| {
			lightmap
				.write_to(file, image::ImageFormat::Png)
				.context("Failed to save image")
		})
	}

	/// Processes a single region file
	fn process_region(&self, coords: TileCoords) -> Result<bool> {
		/// Width/height of the region data
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let mut processed_region = ProcessedRegion::default();
		let mut lightmap = image::GrayAlphaImage::new(N, N);

		let input_path = self.config.region_path(coords);
		let input_timestamp = fs::modified_timestamp(&input_path)?;

		let output_path = self.config.processed_path(coords);
		let output_timestamp = fs::read_timestamp(&output_path, FILE_META_VERSION);
		let lightmap_path = self.config.tile_path(TileKind::Lightmap, 0, coords);
		let lightmap_timestamp = fs::read_timestamp(&lightmap_path, FILE_META_VERSION);

		if Some(input_timestamp) <= output_timestamp && Some(input_timestamp) <= lightmap_timestamp
		{
			debug!("Skipping unchanged region r.{}.{}.mca", coords.x, coords.z);
			return Ok(false);
		}

		debug!("Processing region r.{}.{}.mca", coords.x, coords.z);

		crate::nbt::region::from_file(input_path)?.foreach_chunk(
			|chunk_coords, data: world::de::Chunk| {
				let Some(layer::LayerData {
					blocks,
					biomes,
					block_light,
					depths,
				}) = self
					.process_chunk(&mut processed_region.biome_list, data)
					.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))?
				else {
					return Ok(());
				};
				processed_region.chunks[chunk_coords] = Some(Box::new(ProcessedChunk {
					blocks,
					biomes,
					depths,
				}));

				let chunk_lightmap = Self::render_chunk_lightmap(block_light);
				overlay_chunk(&mut lightmap, &chunk_lightmap, chunk_coords);

				Ok(())
			},
		)?;

		if Some(input_timestamp) > output_timestamp {
			Self::save_region(&output_path, &processed_region, input_timestamp)?;
		}
		if Some(input_timestamp) > lightmap_timestamp {
			Self::save_lightmap(&lightmap_path, &lightmap, input_timestamp)?;
		}

		Ok(true)
	}

	/// Iterates over all region files of a Minecraft save directory
	///
	/// Returns a list of the coordinates of all processed regions
	pub fn run(self) -> Result<Vec<TileCoords>> {
		let mut regions = self.collect_regions()?;

		// Sort regions in a zig-zag pattern to optimize cache usage
		regions.sort_unstable_by_key(|&TileCoords { x, z }| (x, if x % 2 == 0 { z } else { -z }));

		fs::create_dir_all(&self.config.processed_dir)?;
		fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, 0))?;

		info!("Processing region files...");

		let mut results = vec![];
		regions
			.par_iter()
			.map(|&coords| {
				let result = self.process_region(coords);
				if let Err(err) = &result {
					error!("Failed to process region {:?}: {:?}", coords, err);
				}
				result
			})
			.collect_into_vec(&mut results);

		let processed = results
			.iter()
			.filter(|result| matches!(result, Ok(true)))
			.count();
		let errors = results.iter().filter(|result| result.is_err()).count();
		info!(
			"Processed region files ({} processed, {} unchanged, {} errors)",
			processed,
			results.len() - processed,
			errors,
		);

		Ok(regions)
	}
}
