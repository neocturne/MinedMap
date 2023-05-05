use std::fs;

use anyhow::{Context, Result};

use minedmap::{io::storage, resource::block_color, types::*};

use super::common::*;

pub struct TileRenderer<'a> {
	config: &'a Config,
}

impl<'a> TileRenderer<'a> {
	pub fn new(config: &'a Config) -> Self {
		TileRenderer { config }
	}

	fn load_region(&self, coords: RegionCoords) -> Result<ProcessedRegion> {
		let processed_path = self.config.processed_path(coords, false);
		storage::read(&processed_path).context("Failed to load processed region data")
	}

	fn render_chunk(image: &mut image::RgbaImage, coords: ChunkCoords, chunk: &ProcessedChunk) {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		let chunk_image = image::RgbaImage::from_fn(N, N, |x, z| {
			let coords = LayerBlockCoords {
				x: BlockX(x as u8),
				z: BlockZ(z as u8),
			};
			image::Rgba(
				match (
					&chunk.blocks[coords],
					&chunk.biomes[coords],
					&chunk.depths[coords],
				) {
					(Some(block), Some(biome), Some(depth)) => {
						block_color(*block, biome, depth.0 as f32)
					}
					_ => [0, 0, 0, 0],
				},
			)
		});
		overlay_chunk(image, &chunk_image, coords);
	}

	fn render_region(image: &mut image::RgbaImage, region: &ProcessedRegion) {
		for (coords, chunk) in region.iter() {
			let Some(chunk) = chunk else {
				continue;
			};

			Self::render_chunk(image, coords, chunk);
		}
	}

	fn render_tile(&self, coords: RegionCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let tmp_path = self.config.map_path(coords, true);
		let output_path = self.config.map_path(coords, false);
		println!(
			"Rendering tile {}",
			output_path
				.file_name()
				.unwrap_or_default()
				.to_string_lossy(),
		);

		let region = self.load_region(coords)?;
		let mut image = image::RgbaImage::new(N, N);
		Self::render_region(&mut image, &region);
		image
			.save_with_format(&tmp_path, image::ImageFormat::Png)
			.context("Failed to save image")?;
		fs::rename(&tmp_path, &output_path).with_context(|| {
			format!(
				"Failed to rename {} to {}",
				tmp_path.display(),
				output_path.display(),
			)
		})?;

		Ok(())
	}

	pub fn run(self, regions: &[RegionCoords]) -> Result<()> {
		fs::create_dir_all(&self.config.map_dir).with_context(|| {
			format!(
				"Failed to create directory {}",
				self.config.map_dir.display(),
			)
		})?;

		for &coords in regions {
			if let Err(err) = self.render_tile(coords) {
				eprintln!("Failed to render tile {:?}: {:?}", coords, err,);
			}
		}

		Ok(())
	}
}
