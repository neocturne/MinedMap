use std::path::Path;

use anyhow::{Context, Result};

use minedmap::{
	io::{fs, storage},
	resource::block_color,
	types::*,
};

use super::common::*;

pub struct TileRenderer<'a> {
	config: &'a Config,
}

impl<'a> TileRenderer<'a> {
	pub fn new(config: &'a Config) -> Self {
		TileRenderer { config }
	}

	fn load_region(processed_path: &Path) -> Result<ProcessedRegion> {
		storage::read(processed_path).context("Failed to load processed region data")
	}

	fn render_chunk(image: &mut image::RgbaImage, coords: ChunkCoords, chunk: &ProcessedChunk) {
		const N: u32 = BLOCKS_PER_CHUNK as u32;

		let chunk_image = image::RgbaImage::from_fn(N, N, |x, z| {
			let coords = LayerBlockCoords {
				x: BlockX::new(x),
				z: BlockZ::new(z),
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

	fn render_tile(&self, coords: TileCoords) -> Result<()> {
		const N: u32 = (BLOCKS_PER_CHUNK * CHUNKS_PER_REGION) as u32;

		let processed_path = self.config.processed_path(coords);
		let processed_timestamp = fs::modified_timestamp(&processed_path)?;

		let output_path = self.config.tile_path(TileKind::Map, 0, coords);
		let output_timestamp = fs::read_timestamp(&output_path, FILE_META_VERSION);

		if Some(processed_timestamp) <= output_timestamp {
			println!(
				"Skipping unchanged tile {}",
				output_path
					.strip_prefix(&self.config.output_dir)
					.expect("tile path must be in output directory")
					.display(),
			);
			return Ok(());
		}

		println!(
			"Rendering tile {}",
			output_path
				.strip_prefix(&self.config.output_dir)
				.expect("tile path must be in output directory")
				.display(),
		);

		let region = Self::load_region(&processed_path)?;
		let mut image = image::RgbaImage::new(N, N);
		Self::render_region(&mut image, &region);

		fs::create_with_timestamp(
			&output_path,
			FILE_META_VERSION,
			processed_timestamp,
			|file| {
				image
					.write_to(file, image::ImageFormat::Png)
					.context("Failed to save image")
			},
		)
	}

	pub fn run(self, regions: &[TileCoords]) -> Result<()> {
		fs::create_dir_all(&self.config.tile_dir(TileKind::Map, 0))?;

		for &coords in regions {
			if let Err(err) = self.render_tile(coords) {
				eprintln!("Failed to render tile {:?}: {:?}", coords, err);
			}
		}

		Ok(())
	}
}
