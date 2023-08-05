use std::{path::Path, time::SystemTime};

use anyhow::{Context, Result};

use indexmap::IndexSet;
use minedmap::{
	io::{fs, storage},
	resource::{self, Biome},
	types::*,
	world::{
		self,
		layer::{self, LayerData},
	},
};

use super::common::*;

/// Parses a filename in the format r.X.Z.mca into the contained X and Z values
fn parse_region_filename(path: &Path) -> Option<TileCoords> {
	let file_name = path.file_name()?.to_str()?;
	let parts: Vec<_> = file_name.split('.').collect();
	let &["r", x, z, "mca"] = parts.as_slice() else {
		return None;
	};

	Some(TileCoords {
		x: x.parse().ok()?,
		z: z.parse().ok()?,
	})
}

/// Type with methods for processing the regions of a Minecraft save directory
pub struct RegionProcessor<'a> {
	block_types: resource::BlockTypes,
	biome_types: resource::BiomeTypes,
	config: &'a Config,
}

impl<'a> RegionProcessor<'a> {
	pub fn new(config: &'a Config) -> Self {
		RegionProcessor {
			block_types: resource::BlockTypes::default(),
			biome_types: resource::BiomeTypes::default(),
			config,
		}
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

	fn render_chunk_lightmap(
		block_light: Box<world::layer::BlockLightArray>,
	) -> image::GrayAlphaImage {
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

	fn save_region(
		path: &Path,
		processed_region: &ProcessedRegion,
		timestamp: SystemTime,
	) -> Result<()> {
		storage::write(path, processed_region, FILE_META_VERSION, timestamp)
	}

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
	fn process_region(&self, path: &Path, coords: TileCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let mut processed_region = ProcessedRegion::default();
		let mut lightmap = image::GrayAlphaImage::new(N, N);

		let input_timestamp = fs::modified_timestamp(path)?;

		let output_path = self.config.processed_path(coords);
		let output_timestamp = fs::read_timestamp(&output_path, FILE_META_VERSION);
		let lightmap_path = self.config.tile_path(TileKind::Lightmap, 0, coords);
		let lightmap_timestamp = fs::read_timestamp(&lightmap_path, FILE_META_VERSION);

		if Some(input_timestamp) <= output_timestamp && Some(input_timestamp) <= lightmap_timestamp
		{
			println!("Skipping unchanged region r.{}.{}.mca", coords.x, coords.z);
			return Ok(());
		}

		println!("Processing region r.{}.{}.mca", coords.x, coords.z);

		minedmap::io::region::from_file(path)?.foreach_chunk(
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
				processed_region.chunks[chunk_coords] = Some(ProcessedChunk {
					blocks,
					biomes,
					depths,
				});

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

		Ok(())
	}

	/// Iterates over all region files of a Minecraft save directory
	///
	/// Returns a list of the coordinates of all processed regions
	pub fn run(self) -> Result<Vec<TileCoords>> {
		let read_dir = self.config.region_dir.read_dir().with_context(|| {
			format!(
				"Failed to read directory {}",
				self.config.region_dir.display()
			)
		})?;

		fs::create_dir_all(&self.config.processed_dir)?;
		fs::create_dir_all(&self.config.tile_dir(TileKind::Lightmap, 0))?;

		let mut ret = Vec::new();

		for entry in read_dir.filter_map(|entry| entry.ok()).filter(|entry| {
			// We are only interested in regular files
			entry
				.file_type()
				.map(|file_type| file_type.is_file())
				.unwrap_or_default()
		}) {
			let path = entry.path();
			let Some(coords) = parse_region_filename(&path) else {
				continue;
			};

			if let Err(err) = self.process_region(&path, coords) {
				eprintln!(
					"Failed to process region {}: {:?}",
					path.file_name().unwrap_or_default().to_string_lossy(),
					err,
				);
			}

			ret.push(coords);
		}

		Ok(ret)
	}
}
