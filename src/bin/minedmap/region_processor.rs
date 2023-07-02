use std::path::Path;

use anyhow::{Context, Result};

use minedmap::{
	io::{fs, storage},
	resource,
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
	fn process_chunk(&self, data: world::de::Chunk) -> Result<Option<LayerData>> {
		let chunk = world::chunk::Chunk::new(&data, &self.block_types, &self.biome_types)?;
		world::layer::top_layer(&chunk)
	}

	fn render_chunk_lightmap(
		block_light: Box<world::layer::BlockLightArray>,
	) -> image::GrayAlphaImage {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		image::GrayAlphaImage::from_fn(N, N, |x, z| {
			let v: f32 = block_light[LayerBlockCoords {
				x: BlockX(x as u8),
				z: BlockZ(z as u8),
			}]
			.into();
			image::LumaA([0, (192.0 * (1.0 - v / 15.0)) as u8])
		})
	}

	fn save_region(&self, coords: TileCoords, processed_region: &ProcessedRegion) -> Result<()> {
		let output_path = self.config.processed_path(coords);
		storage::write(&output_path, processed_region)
	}

	fn save_lightmap(&self, coords: TileCoords, lightmap: &image::GrayAlphaImage) -> Result<()> {
		fs::create_with_tmpfile(&self.config.light_path(coords), |file| {
			lightmap
				.write_to(file, image::ImageFormat::Png)
				.context("Failed to save image")
		})
	}

	/// Processes a single region file
	fn process_region(&self, path: &Path, coords: TileCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		println!("Processing region r.{}.{}.mca", coords.x, coords.z);

		let mut processed_region = ProcessedRegion::default();
		let mut lightmap = image::GrayAlphaImage::new(N, N);

		minedmap::io::region::from_file(path)?.foreach_chunk(
			|chunk_coords, data: world::de::Chunk| {
				let Some(layer::LayerData{ blocks, biomes, block_light, depths }) = self
					.process_chunk(data)
					.with_context(|| format!("Failed to process chunk {:?}", chunk_coords))?
				else {
					return Ok(());
				};
				processed_region[chunk_coords] = Some(ProcessedChunk {
					blocks,
					biomes,
					depths,
				});

				let chunk_lightmap = Self::render_chunk_lightmap(block_light);
				overlay_chunk(&mut lightmap, &chunk_lightmap, chunk_coords);

				Ok(())
			},
		)?;

		self.save_region(coords, &processed_region)?;
		self.save_lightmap(coords, &lightmap)?;

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
		fs::create_dir_all(&self.config.light_dir)?;

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
